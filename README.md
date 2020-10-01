# Soil-i2c
A tool to measure soil moisture from Adafruit STEMMA Soil Sensors, with support for I2C multiplexer chips built in, as well as support for MySQL and planned future support for Sqlite, PostgreSQL, and ODBC.

## About the Project/DIY Components
If you'd like to replicate the project I'm creating, you'll need the following components:

* 1 Raspberry Pi 3B+ board
* 1 or more Adafruit STEMMA Soil Sensors
* 1 Adafruit JST PH 4-Pin to Male Header cable **PER SENSOR** *(though you could maybe work around this with regular wire)*
* *OPTIONAL:* An Adafruit TCA9548A I2C Multiplexer, if you plan on reading from many sensors like me
* *OPTIONAL:* A MySQL or MariaDB database with the following CREATE TABLE:
  * ID int AUTO_INCREMENT NOT NULL,
  * Plant varchar NOT NULL,
  * Readtime datetime NOT NULL,
  * Moisture int DEFAULT NULL,
  * Temperature float DEFAULT NULL,
  * PRIMARY-KEY(ID)
  
Depending on what you'd like to do with the program, pretty much everything you need is in the source code. With Rust's extensibility, you could put together just about anything with this framework; the **run** function is what gathers the data and uploads it all to the database, so all you'd need to do to hack the program the way you like it is modify that main function to run things the way you'd like. I'm thinking in the future I'd like to set up the sensor activity with parameters rather than a hardcoded loop, but we'll worry about that later. (Already somewhat there with this new run function!)

Currently for MySQL integration, the server URL needs to be hardcoded, so you'll have to change it up depending on what your own URL is. (line 19 in main.rs)
I plan to change this to a config file in the future when the project gets closer to a decent release state.

## To-Do List
* Set the program to run on a defined schedule
* Add support for other databases (Sqlite, ODBC, Postgre)
* Add built-in database/table creation scripts
* Move most user-defined things into a config file (server connection string, frequency of readings, etc.)
 * Possibly add a config-mode or first-time-setup argument to the program to allow for easy setup of the config file
* Look into adding optional distribution of data via OPC or Modbus

## Attribution
This software makes use of several external libraries as dependencies. While such dependencies have not been redistributed in this repository in source form, the copyright information for this program's dependencies is listed below, and each is licensed under the MIT License unless otherwise specified.

**embedded-hal & linux-embedded-hal**
<br>*Copyright (c) 2017-2018 Jorge Aparicio*

**rppal**
<br>*Copyright (c) 2017-2020 Rene van der Meer*

**stemma-soil-sensor**
<br>*Copyright (c) 2020 Carl Fredrik Samson*

**mysql**
<br>*Copyright (c) 2020 rust-mysql-common contributors*

    
