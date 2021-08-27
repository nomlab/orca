// orca : CLI Container management tool
// This program is managemented by nomlab <https://github.com/nomlab>

use orca::args::Args;
use orca::image::Image;
use orca::container::Container;
use orca::terminal::Terminal;
use orca::command::Command;
use dirs::home_dir;

fn main() {

    let default_dest_name = String::from("debian");
    let default_dest_tag = String::from("latest");
    let default_command = String::from("sh");

    let mut args = Args::new();
    args.set_args(); // Set args from stdin into args instanse

    if args.image_name == None {
        args.image_name = Some(default_dest_name);
    }
    if args.image_tag == None {
        args.image_tag = Some(default_dest_tag);
    }
    if args.command == None {
        args.command = Some(default_command);
    }

    let rootfs_path = format!(
        "{}/.local/orca/{}/{}",
        home_dir().unwrap().to_str().unwrap(),
        &args.image_name.as_ref().unwrap(),
        &args.image_tag.as_ref().unwrap()
    );

    let image = Image::new(
        rootfs_path,
        args.image_name.unwrap().to_string(),
        args.image_tag.unwrap().to_string()
    );
    if image.exist() {
        if args.init_flag {
            println!("Remove image already existing");
            image.remove().unwrap();
            println!("Extract image");
            image.extract().unwrap();
        }
    }else{
        println!("Download image");
        image.download().unwrap();
        println!("Extract image");
        image.extract().unwrap();
    }

    let working_container = Container::new(
        image,
        args.command.unwrap().to_string(),
        args.netns_flag
    );

    if Command::new("newuidmap", Option::<Vec<String>>::None).is_exist() {
        working_container.map_id_with_subuid().unwrap();
    } else {
        working_container.map_id().unwrap();
    }

    working_container.connect_tty().unwrap();

    let mut terminal = Terminal::new();
    terminal.into_raw_mode().unwrap();

    let used_image = working_container.wait().unwrap();

    if args.remove_flag {
        used_image.remove().unwrap();
    }
}

