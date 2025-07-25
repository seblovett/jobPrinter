# jobPrinter
A Rust Application to print jobs out to a Thermal Printer
This is heavily inspired by [this post](https://www.laurieherault.com/articles/a-thermal-receipt-printer-cured-my-procrastination) 

My general idea builds on this, but for the routine tasks. 
I bought a cheap [ESC/POS printer](https://www.amazon.co.uk/Thermal-Receipt-Ethernet-Restaurant-Business/dp/B0CM6DHW7R/ref=sr_1_19?crid=20EZSNGJFXCGW&dib=eyJ2IjoiMSJ9.o3fm9-MBbcO4DLSwFI2irzD5-B35HbgUpGa0aOW5WGUZGRhG2wV56EYPd6Xma5--4vxLkAPMKv87JzmzrDnBQU_NSEZtAd-dSgyPmwR-8w_YUlkoGxDDMJjxsGEu24_2NKobcnmuT5NJyGmNMFINJyxK5PgVd8Iji_n6roaFzo799b_4Ewq5olCjUfNDwAveXbJvV2dWG68fGKsIxAXd0dkDDaG4CaDEMMn9e9dy2ad_S_f4G0mABlSbcOIMgLET89VCz3xp5pu898GQrSWRjisIeZZnp0SCTiY5KHdaZr0.34M7ZvbQPcaaO6cEPT-NCR0PAjuhmQCNzXYJqv0KyAU&dib_tag=se&keywords=receipt+printer&qid=1752759570&sprefix=receipt+printer%2Caps%2C95&sr=8-19) to do this (not branded).
It will sit in the kitchen and spit out jobs at specified times, or when sent. 
A RPi will be added to the device to make it fully standalone. 

# Requirements
Using a rough MoSCoW principle. These will undoubtedly change over time.
## Must
- Print routine jobs out at a specific time or interval 
    e.g. Sunday 19:00 - put bins out; Every 5 days, water the plants. 
- Be a standalone system
- Be able to send one off jobs / list of jobs to be printed. 
- Use Rust (I have been learning and want to do a project with it!)

## Should

## Could
- Implement a rewards scheme for completing jobs.
- Have a points tally to keep score
- Power down the printer when not in use

## Won't

# Implementation
--------------
The plan is:
- Host a REST API to submit jobs to (POST)
- On POST, job is stored to a database and printed
- In future, keeps tabs on open jobs and scores. 
- Web interface to show open jobs and a score board


A scheduler will be implemented that keeps jobs with a schedule
It will submit a POST to print the job at the right time.
Schedules should be in CRON format, `0 19 * * 0; Put bins out`
