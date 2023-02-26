# Minosse
Minosse is a simple windows service that monitors running processes and sets their priority and affinity based on a configuration file.

## Usage

1. Download the latest installer from the [releases page](https://github.com/artumino/minosse/releases)
2. Run the installer, it will install the service that reads a configuration file located in `<INSTALL-DIRECTORY>\rules.json`
3. Edit the configuration file to your liking

## Configuration

The configuration file is a JSON file that contains an array of rules. Each rule has the following properties:
- `pattern`: The regex pattern that will be used to match the process name, e.g. `audiodg\.exe`
- `priority`: The priority to set for the process, e.g. `High`
- `affinity`: An array of CPU cores to set the affinity to, e.g. `[0, 1]`
Rules are evaluated in the order they are defined in the configuration file. All rules that matches a process will be applied.

## Example
```json
{
    "rules":
    [
        {
            "pattern": "audiodg\\.exe",
            "priority": "High",
            "affinity": [15]
        }
    ]
}
```

The above configuration is usefull to avoid audio glitches on Ryzen CPUs when using VoiceMeeter Banana in conjuction with Discord.