use std::fs;
use std::io::{Write};
use std::env;
use std::process;
use std::path::PathBuf;

/*
	Commands:
	h : cfs -h [optional;str]command
	s : cfs -s [str]confpath [str]source [str]name
	l : cfs -l [str]name
	r : cfs -r [str]name
	t : cfs -t [str]name1  [str]name2
	ls: cfs -ls
	
	Return points:
	0 - Success
	1 - Failure
	2 - Wrong user input
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {

	// Argument collection
	let args: Vec<String> = env::args().collect();



	// Command validation
	let mut cmd: String = "-h".to_string();
	if args.len() >= 2 { 
		
		cmd = args[1].clone();

		match cmd.as_str() {

			"-s" if args.len() != 5 => { // -s (5 arguments)
				println!("Usage: cfs -s {{path of the config}} \
					{{path of the file you want to save}} \
					{{the name you want to save the config as}}");
				process::exit(2);
			}

			"-l" if args.len() != 3 => { // -l (3 arguments)
				println!("Usage: cfs -l {{config name}}");
				process::exit(2);
			}

			"-r" if args.len() != 3 => { // -r (3 arguments)
				println!("Usage: cfs -r {{config name}}");
				process::exit(2);
			}

			"-t" if args.len() != 4 => { // -t (4 arguments)
				println!("Usage: cfs -t {{1st config name}} {{2nd config name}}");
				process::exit(2);
			}

			_ => { }
		}
	}
	


	// Directory management
	let home = env::var("HOME").expect("HOME environment variable not set"); // home
	
	let mut data_path = PathBuf::from(&home); // local appdata
	data_path.push(".local");
	data_path.push("share");
	data_path.push("cfs");
	fs::create_dir_all(&data_path).expect("Failed to create data directory"); // create local appdata

	let mut conf_dir = PathBuf::from(&home); // config directory
	conf_dir.push(".config");



	// Command execution

	// -h : help menu
	if cmd == "-h" {

		let helpmenu = r#"
~#- COMMANDS -#~

 -h  : Shows this menu
Example: cfs -h

 -s  : Saves a config with the path of config you want to replace, file you want to use, provided name
Example: cfs -s hypr/hyprland.conf somefile.txt mainhyprlandconfig

 -l  : Loads a config with the provided name
Example: cfs -l mainhyprlandconfig

 -r  : Deletes a config with the provided name
Example: cfs -r mainhyprlandconfig

 -t  : Toggles between 2 configs, both of which you have to provide names for
Example: cfs -t mainhyprlandconfig 2ndconfig

 -ls : Lists all saved configs
Example: cfs -ls
"#;

		println!("{}", helpmenu); // print menu
	}

	// -s : saving
	else if cmd == "-s" {

		/*
			arg 2 : config path
			arg 3 : source name
			arg 4 : saved config name
		*/

    let contents = fs::read_to_string(&args[3])?; // load source
    data_path.push(&args[4]);

    let mut config = fs::File::create(data_path)?; // create file
    writeln!(config, "{}", &args[2])?; // write file metadata
    write!(config, "{}", contents)?; // copy original file
	}

	// -l : loading
	else if cmd == "-l" {

		/*
			arg 2 : saved config name to load
		*/

		data_path.push(&args[2]); // add name to path
    let contents = fs::read_to_string(data_path)?; // load config
    let mut lines = contents.lines(); // creates an iterator over each line
    let conf_path = lines.next().unwrap_or_default(); // config load path
    let file_contents = lines.collect::<Vec<&str>>().join("\n"); // rest of the file

    conf_dir.push(conf_path); // config/loading path

    let mut config = fs::File::create(conf_dir)?; // create file
    write!(config, "{}", file_contents)?; // copy config file without metadata
	}

	// -r : remove
	else if cmd == "-r" {
		// arg 2 : config name to delete
		let mut path_to_delete = data_path.clone();
		path_to_delete.push(&args[2]);
	
		if path_to_delete.exists() {
			fs::remove_file(&path_to_delete)?;
			println!("Deleted config: {}", &args[2]);
		} else {
			eprintln!("Config {} does not exist!", &args[2]);
			process::exit(1);
		}
	}
	
	// -t : toggle
	else if cmd == "-t" {
	
		/*
			arg 2 : config nr 1
			arg 3 : config nr 2
		*/

		let conf1 = data_path.join(&args[2]);
		let conf2 = data_path.join(&args[3]);

		let contents_file1 = fs::read_to_string(&conf1)?;
		let contents_file2 = fs::read_to_string(&conf2)?;

		let first_line_conf1 = contents_file1.lines().next().unwrap_or_default();
		let first_line_conf2 = contents_file2.lines().next().unwrap_or_default();

		if first_line_conf1 != first_line_conf2 {
			eprintln!("Config first lines mismatch!");
			process::exit(1);
		}

		let conf_dir = PathBuf::from(home).join(".config").join(first_line_conf1);

		let target = if contents_file1 == fs::read_to_string(&conf_dir)? {
			conf2
		} else {
			conf1
		};
		let contents = fs::read_to_string(&target)?;

		let mut file = fs::File::create(&conf_dir)?;
		writeln!(file, "{}", first_line_conf1)?;
		write!(file, "{}", contents)?;
	}
	
	// -ls: list
	else if cmd == "-ls" {
		for entry in fs::read_dir(&data_path)? { // loop through all configs
			let entry = entry?; // entry thing
			let path = entry.path(); // make it a path
			let filename = path.file_name().unwrap().to_string_lossy(); // get filename (used for listing)
			
			let contents = fs::read_to_string(&path)?; // keep the String alive
			let first_line = contents.lines().next().unwrap_or_default();
			
			println!("{} -> {}", filename, first_line); // print output
		}
	}

	Ok(())
}
