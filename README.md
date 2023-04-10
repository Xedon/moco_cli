# Moco CLI
Provide CRUD CLI for Moco Activities with Jira Cloud Sync Option for faster time tracking.

## How to install

Download [latest release asset](https://github.com/Xedon/moco_cli/releases) for your architecture and name the binary as u want.
Moco CLI only requieres libc which is in most cases allready installed.

## Available commands

```
Moco CLI

Usage: mococli [OPTIONS] <COMMAND>

Commands:
  login  Login into (Moco/Jira)
  list   List activities
  new    Create new activity
  edit   Edit activity
  rm     Delete activity
  timer  Start/Stop activity timer
  sync   Sync missing Jira Tempo logs to Moco
  help   Print this message or the help of the given subcommand(s)

Options:
      --debug  Show additional information for bug reports
  -h, --help   Print help
```

### Login

```
Login into (Moco/Jira)

Usage: mococli login [SYSTEM]

Arguments:
  [SYSTEM]  [default: moco] [possible values: moco, jira]

Options:
  -h, --help  Print help
```

### List

```
List activities

Usage: mococli list [OPTIONS]

Options:
      --today       
      --week        
      --last-week   
      --month       
      --last-month  
      --compact     Sum up all activities of the day to one entry
  -h, --help        Print help
```

### New

```
Create new activity

Usage: mococli new [OPTIONS]

Options:
      --project <PROJECT>          Optional project id for the activity
      --task <TASK>                Optional task id for the activity
      --hours <HOURS>              Optional hours in format (h.m)
      --date <DATE>                Optional date in format (YYYY-mm-dd)
      --description <DESCRIPTION>  Optional description for the activity
  -h, --help
```

### Edit

```
Edit activity

Usage: mococli edit [OPTIONS]

Options:
      --activity <ACTIVITY>  Optional activity id
  -h, --help                 Print help
```

### Rm

```
Delete activity

Usage: mococli rm [OPTIONS]

Options:
      --activity <ACTIVITY>  Optional activity id
  -h, --help                 Print help
```

### Timer

```
Start/Stop activity timer

Usage: mococli timer [OPTIONS] <SYSTEM>

Arguments:
  <SYSTEM>  [possible values: start, stop]

Options:
      --activity <ACTIVITY>  Optional activity id
  -h, --help                 Print help
```

### Sync (Currently only creation of activities is supported)

```
Sync missing Jira Tempo logs to Moco

Usage: mococli sync [OPTIONS] [SYSTEM]

Arguments:
  [SYSTEM]  [default: jira] [possible values: jira]

Options:
      --today              
      --week               
      --last-week          
      --month              
      --last-month         
      --project <PROJECT>  Optional project id for the activity
      --task <TASK>        Optional task id for the activity
      --dry-run            Just list what will be booked in moco from Jira
  -h, --help               Print help
```