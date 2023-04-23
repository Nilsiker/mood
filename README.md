# mood
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-informational)](COPYRIGHT.md)

`mood` is a minimal journaling CLI, inspired by the Android app [Pixels, by Teo Vogel](https://play.google.com/store/apps/details?id=ar.teovogel.yip).

In order to be as lean and unintrusive as possible, `mood` only provides one generative operation: **writing or overwriting the entry of today**. No bells and whistles included, keep it simple and jot down your thought(s).

## Features

* Add minimal journal entries through CLI commands
    * Rate the day on a 1-5 scale (Terrible, Bad, Neutral, Good, Great)
    * Add an optional note to help remember what made the day what it was.
* Get entries specific to a date, or certain date ranges
* Stores a journal file locally in a human-readable RON format

## Installing
You can either build this program from source, or install it using 

`cargo install mood`

## Usage

To explore the commands and options in the CLI, run `mood -h` in your terminal.

By default, `mood` stores a journal file right alongside the executable file. If you wish to configure where this file is stored (for example, if you wish to put it in a cloud-synced folder), you can use:

`mood config -p <path_to_file>`

### Example: Add an entry

To add a daily journal, use the command below. If you wish to update the entry, simply rerun the 

`mood add <RATING> <OPTIONAL NOTE>`


## A box of future ideas

* Data visualization commands (graphs in terminals, insights relating to moods and note keywords)
* Optional colorization depending on rating
* Custom mood ratings
* Structured notes option (customizable emotions and/or activity names)
* More ergonomic notes section using an indefinite amount of argument values, instead of text content wrapped in citation marks.
* Customizable journal file path.