use crate::io::output::BOLD;
use crate::io::output::RESET;

pub fn printhelp() {
	println!("{}", [
		"SMAIL -- command line tool for sending HTML e-mails",
		"",
		&[ "Usage: ", BOLD, "smail", RESET," [flags] <command> ..." ].join(""),
		"",
		"Possible flags:",
		"\t--config=/path/to/config - specifies a custom config file path,",
		"\t                           otherwise ~/.smailconf will be used",
		"",
		"Possible commands:",
		&[ "\tsmail ", BOLD, "init", RESET, " [--overwrite] - create a new configuration file" ].join(""),
		"\t\t--overwrite = forces the creation of a configuration file,",
		"\t\t              even if it already exists",
		"",
		&[ "\tsmail ", BOLD, "create", RESET, " <filename> <?filename> ... - creates empty .smail template(s)" ].join(""),
		"",
		&[ "\tsmail ", BOLD, "send", RESET, " <filename> <?filename> ... - sends filled in .smail file(s)" ].join(""),
		"",
		&[ "\tsmail ", BOLD, "send", RESET, " - sends a filled SMAIL file from stdin" ].join(""),
		"",
		&[ "\tsmail --stdin=\"...\" ", BOLD, "send", RESET ].join(""),
		"\t\t- sends a filled SMAIL file passed as raw text in the command line",
		"\t\t  argument, lines separated with \"\\n\"",
		"",
		&[ "\tsmail --stdin=\"...\" --argument-override-newline=\"delim\" ", BOLD, "send", RESET ].join(""),
		"\t\t- sends a filled SMAIL file passed as raw text in the command line",
		"\t\t  argument, lines separated with the custom delimiter",
		"",
		"For more information about SMAIL, please visit the project's repository:",
		"\thttps://github.com/omekina/smail"
	].join("\n"));
}
