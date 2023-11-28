<p align="center"><img src="smail.webp" width="400px" /></p>

# SMAIL
SMAIL is a command line tool for sending HTML e-mails.
It uses secured ESMTP to communicate with the server and you can even store your
credentials on your drive in the `~/.smailconf` file. _In the future, I would also like
to find some time to encrypt the credentials using a master key._

## Basic introduction - Let's send a simple e-mail
Okay. What will we need to do?
1. Create config.
2. Create `.smail` file to test on.
3. Change the mail file a bit.
4. Send the e-mail.
5. That's it. It is that simple.

### 1. Create config
Let's run:
```shell
smail init
```
This will guide us through config file creation. If you wish you can change it manually.
The config file should be located in `~/.smailconf`.

The `smail init` command will prompt for your password and the password will
be written in plaintext into the config file. So please keep that in mind.

### 2. Create a test mail
To create a simple `.smail` file you can run:
```shell
smail create <filename> <?filename> <?filename> ...
```
This will create a file with template in `<filename>.smail` for each entry. When using SMAIL normally you can
understandably skip this step and create the file manually.

### 3. Change the mail file
The default file is just a template. It should look like this:
```text
==SUBJECT
Test e-mail
==FROM
Alice Smith;alice@nonexistent.domain
==TO
bob@nonexistent.domain
John Smith;john@smith.domain
==STYLE
body {
    font-family: Arial;
}
==BODY
<h1>Hello, world!</h1>
<p>This is a test mail file from SMAIL.</p>
<p>
    You can find SMAIL source code
    <a href=\"https://github.com/omekina/smail\">here</a>
    if you are interested. &#128522;
</p>
```

Now you need to change the individual fields. Or you can keep them and watch the e-mail get lost somewhere in the cloud. :)
The fields: _SUBJECT_, _FROM_ and _TO_ are special. They are directly processed by the executable.
So please keep them clean. You can work as you please with the other fields. (for more info refer to the documentation bellow)

The fields _FROM_ and _TO_ can contain either just some e-mail or name and e-mail. If you wish
to include name you can use the following syntax: `Some Name;someemail@some.domain` (name and e-mail are
separated by delimiter `;` character)

It is also worth mentioning that the field _TO_ can contain multiple addresses. And they should be separated by the `\n` (newline) character.

### 4. Let's send it
```shell
smail send <filename> <?filename> <?filename> ...
```
Specify the names of the mail files (again... without the `.smail` extension) you just created after the send command.
And there it is.

If everything went well you should see something like this:
```text
Connected
Completed ESMTP handshake
Logged in

Mail sent:
Test e-mail
```
Of course, this is going to vary depending on the amount of files being sent and their subjects (if nothing goes wrong). :)

## How SMAIL works

### Init
SMAIL init is a nice config creator tool. It creates a config file
at filepath `~/.smailconf`. Then the init command asks you about some credentials
and settings to write into the file.

SMAIL also checks if the file exists, so it does not overwrite.
```shell
smail init
```

If you want to overwrite existing config file you can use:
```shell
smail init --overwrite
```

### Custom config location
If you wish to override the default `~/.smailconf` location you can use the `--config` flag.
You must use absolute path when specifying custom config.
```shell
smail --config=/home/user/email/personal/.smailconf send <filename> <?filename> <?filename> ...
```

The custom config flag works on any smail command which uses the config file.
```shell
smail --config=/home/user/email/work/.smailconf init
```

### SMAIL file structure
This command creates a new e-mail file in your current working directory
(so remember to `cd` properly).
SMAIL e-mail files have file extension `.smail`.
E-Mail files should have the following structure:
```text
==SUBJECT
Test e-mail
==FROM
Alice Smith;alice@nonexistent.domain
==TO
bob@nonexistent.domain
John Smith;john@smith.domain
==STYLE
body {
    font-family: Arial;
}
==BODY
<h1>Hello, world!</h1>
<p>This is a test mail file from SMAIL.</p>
<p>
    You can find SMAIL source code
    <a href=\"https://github.com/omekina/smail\">here</a>
    if you are interested. &#128522;
</p>
```

The create command just creates a template for your e-mail in the `.smail` file,
so you have some starting point.

The `==<name>` marked parts are sections.
Sections are basically search-and-replace items. Section name is looked up in the
configuration file template and replaced with the section from the current e-mail file.

To see more info about this. Take a look into newly created configuration file (init).

### Piping `.smail` files from other programs
SMAIL can also accept `.smail` files from stdin. If no filepath is specified with the
send command, SMAIL will default to reading from stdin.
```shell
cat test.smail | smail send
```
The command above can be achieved with SMAIL alone, but if you wish to generate a file
on the fly and send it immediately... this might come in handy.

### Create
`smail create <filename>` will create a file at `<filename>.smail`.
The file will have some predefined contents (which you can then change).
As we discussed in the _file structure_ chapter, it is not very hard to structure a
`.smail` file. You can create one by hand, but the
```shell
smail create <filename> <?filename> <?filename> ...
```
command will help to get you started.

TIP: Do not specify the `.smail` file extension. As I already mentioned, this
extension will be added by SMAIL.


### Send
`smail send <filename> <?filename> <?filename> ...` will send the mail according to settings in config file
and the contents of the target `.smail` file. Do not specify the `.smail` file extension
as it is added automatically.

## How to pronounce SMAIL
Since quite a lot of software defines pronunciation, I thought SMAIL should too.

This is the official pronunciation:

`/siː-miːl/` or `sea-meal`

Or just pronounce it how you like, I don't care.
