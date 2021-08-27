use clap::{App, Arg};

pub struct Args {
    pub image_name: Option<String>,
    pub image_tag: Option<String>,
    pub command: Option<String>,
    pub init_flag: bool,
    pub remove_flag: bool,
    pub netns_flag: bool,
}

impl Args {
    pub fn new() -> Args {
        Args {
            image_name: None,
            image_tag: None,
            command: None,
            init_flag: false,
            remove_flag: false,
            netns_flag: false
        }
    }

    pub fn set_args(&mut self){
        let app = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(Arg::with_name("image")
                 .short("d")
                 .long("image")
                 .help("Image name of container image")
                 .takes_value(true)
                )
            .arg(Arg::with_name("tag")
                 .short("t")
                 .long("tag")
                 .help("Tag name of container iamge")
                 .takes_value(true)
                )
            .arg(Arg::with_name("init")
                 .short("i")
                 .long("init")
                 .help("Initialize contaier image before running a command")
                )
            .arg(Arg::with_name("remove")
                 .short("r")
                 .long("remove")
                 .help("Remove container image after running a command")
                )
            .arg(Arg::with_name("use_netns")
                 .short("n")
                 .long("netns")
                 .help("Isolate network namespace")
                )
            .arg(Arg::with_name("COMMAND")
                 .help("Command to execute in container")
                );

        let matches = app.get_matches();

        if let Some(o) = matches.value_of("image") {
            self.image_name = Some(o.to_string());
        }
        if let Some(o) = matches.value_of("tag") {
            self.image_tag = Some(o.to_string());
        }
        if let Some(o) = matches.value_of("COMMAND") {
            self.command = Some(o.to_string());
        }
        self.init_flag     = matches.is_present("init");
        self.remove_flag   = matches.is_present("remove");
        self.netns_flag = matches.is_present("use_netns");
    }
}

