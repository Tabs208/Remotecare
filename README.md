#  RemoteCare CLI-Emergency Medical Assistance For Remote Areas.
## Overview 
**RemoteCare** is a command-line tool designed to enhance emergency medical assistance in remote areas. It enables community health workers or locals to log medical emergencies, prioritize cases based on severity, notify nearby healthcare providers via SMS or email, and access offline first-aid guides. This tool aims to streamline emergency response and improve healthcare outcomes in underserved regions.
## Problem Statement:
In rural and remote areas, access to timely emergency medical care is hindered by:
**1.** Poor communication infrastructure.
**2.** Lack of streamlined emergency logging and prioritization.
**3.** Limited healthcare facilities and professionals.

### Solution:
RemoteCare  provides a lightweight, accessible solution to:
**1.Log Medical Emergencies:** Record patient details, location, symptoms, and severity.
**2.Prioritize Emergencies:** Sort cases by severity to address urgent needs first.
**3.Notify Healthcare Providers:** Send alerts via SMS or email to nearby medical centers.
**4.Provide Offline First-Aid Guidance:** Offer text-based first-aid instructions for common emergencies.

### Core Features
**1.Log Medical Emergencies:**
Input patient details, location, symptoms, and severity level.
Save these logs locally in a JSON file or lightweight database (e.g., SQLite).
**example:**
bash  .\target\debug\remotecare.exe log --name "John Doe" --location "Village A" --symptoms "Severe bleeding" --severity High
**output** Emergency logged successfully!
**2.Prioritize Cases:**
List and sort emergency cases by severity to ensure urgent cases receive attention first.
**example:**
bash .\target\debug\remotecare.exe prioritize
**output**
1. John Doe - Severe bleeding (High)
2. Jane Smith - High fever (Medium)
3. Alex Brown - Mild headache (Low)

**3.Notify Healthcare Centers:**
Generate a summary of emergency cases and send them via email or SMS to nearby healthcare providers.
Integration options:
Use Twilio for SMS notifications.
Use an SMTP server (e.g., Gmail) for email.

**4.Offline First Aid Guide:**
Provide text-based first aid instructions for common emergencies like burns, fractures, or fevers:
**example**
bash .\target\debug\remotecare.exe guide --topic burns
**output**
First Aid for Burns:
- Cool the burn under running water for at least 10 minutes.
- Cover the burn with a clean, non-fluffy cloth.
- Do not apply creams or lotions.

### Technical Details
**Programming Language:** Rust 
**Data Storage:** JSON or SQLite for storing emergency logs.
**Notification Integrations:**
  **-Twilio API:** To send SMS alerts.
  **-SMTP Server:** To send email notifications.

### Installation & Setup
**Prerequisites**
Ensure  Rust) is installed.
Install dependencies.
**Installation**
Clone the repository:
 -git clone https://github.com/Tabs208/Remotecare.git
 -cd remotecare
Run the setup:

## Marketing Website
The purpose of the marketing website is to put the idea and the project out there so that it can attract the target group thats is people in rural and remote areas.The website is still under development not fully functionable to the desired purpose.I will continue working on it to a point of integrating the tool into EHR SYSTEM.Below is the link to the website 
[RemoteCare Website](https://Tabs208.github.io/remotecare-website/).
