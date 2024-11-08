# System Monitor Application
User Guide:
How to use Application
1. Launch Application
2. See all the Display
	- Host name: The name of your computer.
	- System name: Your OS system and its version.
    - CPU usage: The current percentage of CPU being used. Lower percentage show minimal usage, while higher percentage show huge consumption of resource.
    - Memory usage: The amount of RAM currently in use out of total available memory, displayed in megabytes (MB).
    - Disk usage: The amount of used disk space compared to the total disk capacity, displayed in gigabytes (GB).
    - Network: The amount of data received in and out in kilobytes (KB)
3. Logging data 
    The application logs system metrics in a CSV file named “system_data.csv” in the same directory and can be used to further analysis. You can access the file through the command prompt.
                cd path/to/your/directory
                type system_data.csv
These will display all the entire contents of the file. If you want to make it easier to read in excel
	start excel system_data.csv

## Installation Instructions

### 1. Clone the Repository
```bash
git clone https://github.com/F3ll4s/System-Monitor.git
cd System-Monitor


### 
