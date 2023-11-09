# SMAIL
SMAIL is a command line tool for sending HTML e-mails.
It uses secured ESMTP to communicate with the server and you can even store your
credentials on your drive in the `~/.smailconf` file. _In the future, I would also like
to find some time to encrypt the credentials using a master key._

## How to pronounce SMAIL
Since quite a lot of software defines pronunciation, I thought SMAIL should too.

This is the official pronunciation:

`/siː-miːl/` or `sea-meal`

Or just pronounce it how you like, I don't care.

## How to use SMAIL

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

### SMAIL file structure
This command creates a new e-mail file in your current working directory
(so remember to `cd` properly).
SMAIL e-mail files have file extension `.smail`.
E-Mail files should have the following structure:
```text
==STYLE
body {
    *something*
}
*something else*

==BODY
<h1>Test SMAIL</h1>
<p>Not lorem ipsum...</p>
*rest of the body*
```

The create command just creates a template for your e-mail in the `.smail` file,
so you have some starting point.

The `==<name>` marked parts are sections.
Sections are basically search-and-replace items. Section name is looked up in the
configuration file template and replaced with the section from the current e-mail file.

To see more info about this. Take a look into newly created configuration file (init).


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
