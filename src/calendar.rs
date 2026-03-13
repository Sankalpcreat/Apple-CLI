use anyhow::Result;
use serde_json::json;

use crate::common::{parse_records, run_applescript, FS, RS};
use crate::{
    CalendarAlarmsAddArgs, CalendarAlarmsDeleteArgs, CalendarAlarmsListArgs, CalendarAttendeesAddArgs,
    CalendarAttendeesListArgs, CalendarCalendarsCreateArgs, CalendarCalendarsDeleteArgs,
    CalendarCreateArgs, CalendarDeleteArgs, CalendarEventsArgs, CalendarGetArgs, CalendarShowArgs,
    CalendarUpdateArgs,
};

pub fn calendar_calendars() -> Result<()> {
    let script = r#"
on run argv
    tell application "Calendar"
        set rs to character id 30
        set outList to {}
        repeat with c in calendars
            set end of outList to (name of c as string)
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[])?;
    let items: Vec<_> = output
        .split(RS)
        .filter(|s| !s.is_empty())
        .map(|name| json!({ "name": name }))
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn calendar_calendars_create(args: CalendarCalendarsCreateArgs) -> Result<()> {
    let name = args.name;
    let description = args.description.unwrap_or_default();
    let script = r#"
on run argv
    set calName to item 1 of argv
    set calDesc to item 2 of argv
    tell application "Calendar"
        set newCal to make new calendar with properties {name:calName}
        if calDesc is not "" then set description of newCal to calDesc
        return (name of newCal as string)
    end tell
end run
"#;
    let output = run_applescript(script, &[name, description])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn calendar_calendars_delete(args: CalendarCalendarsDeleteArgs) -> Result<()> {
    let name = args.name;
    let script = r#"
on run argv
    set calName to item 1 of argv
    tell application "Calendar"
        if not (exists calendar calName) then error "Calendar not found: " & calName
        delete calendar calName
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[name])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn calendar_events(args: CalendarEventsArgs) -> Result<()> {
    let calendar = args.calendar.unwrap_or_default();
    let start = args.start.unwrap_or_default();
    let end = args.end.unwrap_or_default();
    let limit = args.limit.to_string();
    let query = args.query.unwrap_or_default();
    let script = r#"
use scripting additions
on parse_date(dateText)
    if dateText is "" then return missing value
    set datePart to dateText
    set timePart to ""
    if dateText contains " " then
        set AppleScript's text item delimiters to " "
        set parts to text items of dateText
        set AppleScript's text item delimiters to ""
        if (count of parts) ≥ 2 then
            set datePart to item 1 of parts
            set timePart to item 2 of parts
        end if
    end if
    set AppleScript's text item delimiters to "-"
    set dparts to text items of datePart
    set AppleScript's text item delimiters to ""
    if (count of dparts) is not 3 then return missing value
    set y to item 1 of dparts as integer
    set m to item 2 of dparts as integer
    set dy to item 3 of dparts as integer
    tell (current application)
        set d to current date
    end tell
    set year of d to y
    set month of d to m
    set day of d to dy
    if timePart is not "" then
        set AppleScript's text item delimiters to ":"
        set tparts to text items of timePart
        set AppleScript's text item delimiters to ""
        if (count of tparts) ≥ 2 then
            set h to item 1 of tparts as integer
            set mi to item 2 of tparts as integer
            set s to 0
            if (count of tparts) ≥ 3 then set s to item 3 of tparts as integer
            set time of d to (h * 3600 + mi * 60 + s)
        end if
    else
        set time of d to 0
    end if
    return d
end parse_date

on run argv
    set calendarName to item 1 of argv
    set startText to item 2 of argv
    set endText to item 3 of argv
    set limitText to item 4 of argv
    set queryText to item 5 of argv
    if limitText is "" then
        set maxCount to 0
    else
        set maxCount to limitText as integer
    end if
    tell application "Calendar"
        if calendarName is "" then
            set targetCalendar to calendar 1
        else
            if not (exists calendar calendarName) then error "Calendar not found: " & calendarName
            set targetCalendar to calendar calendarName
        end if
        if startText is "" then
            set startDate to current date
            set time of startDate to 0
        else
            set startDate to my parse_date(startText)
            if startDate is missing value then error "Invalid start date"
        end if
        if endText is "" then
            set endDate to startDate + (1 * days)
        else
            set endDate to my parse_date(endText)
            if endDate is missing value then error "Invalid end date"
        end if
        if queryText is "" then
            set matches to (every event of targetCalendar whose start date is greater than or equal to startDate and start date is less than endDate)
        else
            set matches to (every event of targetCalendar whose start date is greater than or equal to startDate and start date is less than endDate and summary contains queryText)
        end if
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with e in matches
            set startTextOut to ""
            set endTextOut to ""
            try
                if start date of e is not missing value then set startTextOut to (start date of e as string)
            end try
            try
                if end date of e is not missing value then set endTextOut to (end date of e as string)
            end try
            set locText to ""
            try
                set locText to (location of e as string)
            end try
            set urlText to ""
            try
                set urlText to (url of e as string)
            end try
            set alldayText to ""
            try
                set alldayText to (allday event of e as string)
            end try
            set rec to (uid of e as string) & fs & (summary of e as string) & fs & startTextOut & fs & endTextOut & fs & locText & fs & urlText & fs & alldayText
            set end of outList to rec
            if maxCount is not 0 then
                if (count of outList) >= maxCount then exit repeat
            end if
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[calendar, start, end, limit, query])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let id = r.get(0).cloned().unwrap_or_default();
            let title = r.get(1).cloned().unwrap_or_default();
            let start = r.get(2).cloned().unwrap_or_default();
            let end = r.get(3).cloned().unwrap_or_default();
            let location = r.get(4).cloned().unwrap_or_default();
            let url = r.get(5).cloned().unwrap_or_default();
            let allday = r.get(6).cloned().unwrap_or_default();
            json!({
                "id": id,
                "title": title,
                "start": start,
                "end": end,
                "location": location,
                "url": url,
                "allday": allday
            })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn calendar_get(args: CalendarGetArgs) -> Result<()> {
    let script = r#"
on run argv
    set eventId to item 1 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        set e to foundEvent
        set fs to character id 31
        set startTextOut to ""
        set endTextOut to ""
        try
            if start date of e is not missing value then set startTextOut to (start date of e as string)
        end try
        try
            if end date of e is not missing value then set endTextOut to (end date of e as string)
        end try
        set locText to ""
        try
            set locText to (location of e as string)
        end try
        set urlText to ""
        try
            set urlText to (url of e as string)
        end try
        set recurrenceText to ""
        try
            set recurrenceText to (recurrence of e as string)
        end try
        set statusText to ""
        try
            set statusText to (status of e as string)
        end try
        set stampText to ""
        try
            set stampText to (stamp date of e as string)
        end try
        set notesText to ""
        try
            set notesText to (description of e as string)
        end try
        set alldayText to ""
        try
            set alldayText to (allday event of e as string)
        end try
        set calName to ""
        try
            set calName to (name of (container of e) as string)
        end try
        return (uid of e as string) & fs & (summary of e as string) & fs & calName & fs & startTextOut & fs & endTextOut & fs & locText & fs & urlText & fs & recurrenceText & fs & statusText & fs & stampText & fs & notesText & fs & alldayText
    end tell
end run
"#;
    let output = run_applescript(script, &[args.id])?;
    let fields: Vec<String> = output.split(FS).map(|f| f.to_string()).collect();
    let id = fields.get(0).cloned().unwrap_or_default();
    let title = fields.get(1).cloned().unwrap_or_default();
    let calendar = fields.get(2).cloned().unwrap_or_default();
    let start = fields.get(3).cloned().unwrap_or_default();
    let end = fields.get(4).cloned().unwrap_or_default();
    let location = fields.get(5).cloned().unwrap_or_default();
    let url = fields.get(6).cloned().unwrap_or_default();
    let recurrence = fields.get(7).cloned().unwrap_or_default();
    let status = fields.get(8).cloned().unwrap_or_default();
    let stamp_date = fields.get(9).cloned().unwrap_or_default();
    let notes = fields.get(10).cloned().unwrap_or_default();
    let allday = fields.get(11).cloned().unwrap_or_default();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "id": id,
            "title": title,
            "calendar": calendar,
            "start": start,
            "end": end,
            "location": location,
            "url": url,
            "recurrence": recurrence,
            "status": status,
            "stamp_date": stamp_date,
            "notes": notes,
            "allday": allday
        }))?
    );
    Ok(())
}

pub fn calendar_create(args: CalendarCreateArgs) -> Result<()> {
    let calendar = args.calendar.unwrap_or_default();
    let title = args.title;
    let start = args.start;
    let end = args.end.unwrap_or_default();
    let allday = if args.allday { "true" } else { "false" }.to_string();
    let location = args.location.unwrap_or_default();
    let notes = args.notes.unwrap_or_default();
    let url = args.url.unwrap_or_default();
    let recurrence = args.recurrence.unwrap_or_default();
    let status = args.status.unwrap_or_default();
    let script = r#"
use scripting additions
on parse_date(dateText)
    if dateText is "" then return missing value
    set datePart to dateText
    set timePart to ""
    if dateText contains " " then
        set AppleScript's text item delimiters to " "
        set parts to text items of dateText
        set AppleScript's text item delimiters to ""
        if (count of parts) ≥ 2 then
            set datePart to item 1 of parts
            set timePart to item 2 of parts
        end if
    end if
    set AppleScript's text item delimiters to "-"
    set dparts to text items of datePart
    set AppleScript's text item delimiters to ""
    if (count of dparts) is not 3 then return missing value
    set y to item 1 of dparts as integer
    set m to item 2 of dparts as integer
    set dy to item 3 of dparts as integer
    tell (current application)
        set d to current date
    end tell
    set year of d to y
    set month of d to m
    set day of d to dy
    if timePart is not "" then
        set AppleScript's text item delimiters to ":"
        set tparts to text items of timePart
        set AppleScript's text item delimiters to ""
        if (count of tparts) ≥ 2 then
            set h to item 1 of tparts as integer
            set mi to item 2 of tparts as integer
            set s to 0
            if (count of tparts) ≥ 3 then set s to item 3 of tparts as integer
            set time of d to (h * 3600 + mi * 60 + s)
        end if
    else
        set time of d to 0
    end if
    return d
end parse_date

on run argv
    set calendarName to item 1 of argv
    set summaryText to item 2 of argv
    set startText to item 3 of argv
    set endText to item 4 of argv
    set alldayText to item 5 of argv
    set locationText to item 6 of argv
    set notesText to item 7 of argv
    set urlText to item 8 of argv
    set recurrenceText to item 9 of argv
    set statusText to item 10 of argv
    tell application "Calendar"
        if calendarName is "" then
            set targetCalendar to calendar 1
        else
            if not (exists calendar calendarName) then error "Calendar not found: " & calendarName
            set targetCalendar to calendar calendarName
        end if
        set startDateVal to my parse_date(startText)
        if startDateVal is missing value then error "Invalid start date"
        set props to {summary:summaryText, start date:startDateVal}
        if endText is not "" then
            set endDateVal to my parse_date(endText)
            if endDateVal is missing value then error "Invalid end date"
            set props to props & {end date:endDateVal}
        end if
        if alldayText is "true" then set props to props & {allday event:true}
        if locationText is not "" then set props to props & {location:locationText}
        if notesText is not "" then set props to props & {description:notesText}
        if urlText is not "" then set props to props & {url:urlText}
        set newEvent to make new event at targetCalendar with properties props
        if recurrenceText is not "" then set recurrence of newEvent to recurrenceText
        if statusText is "confirmed" then
            try
                set status of newEvent to confirmed
            end try
        end if
        if statusText is "tentative" then
            try
                set status of newEvent to tentative
            end try
        end if
        if statusText is "cancelled" then
            try
                set status of newEvent to cancelled
            end try
        end if
        return (uid of newEvent as string)
    end tell
end run
"#;
    let output = run_applescript(
        script,
        &[
            calendar, title, start, end, allday, location, notes, url, recurrence, status,
        ],
    )?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn calendar_update(args: CalendarUpdateArgs) -> Result<()> {
    let title = args.title.unwrap_or_default();
    let start = args.start.unwrap_or_default();
    let end = args.end.unwrap_or_default();
    let allday = args
        .allday
        .map(|v| if v { "true" } else { "false" }.to_string())
        .unwrap_or_default();
    let location = args.location.unwrap_or_default();
    let notes = args.notes.unwrap_or_default();
    let url = args.url.unwrap_or_default();
    let recurrence = args.recurrence.unwrap_or_default();
    let status = args.status.unwrap_or_default();
    let script = r#"
use scripting additions
on parse_date(dateText)
    if dateText is "" then return missing value
    set datePart to dateText
    set timePart to ""
    if dateText contains " " then
        set AppleScript's text item delimiters to " "
        set parts to text items of dateText
        set AppleScript's text item delimiters to ""
        if (count of parts) ≥ 2 then
            set datePart to item 1 of parts
            set timePart to item 2 of parts
        end if
    end if
    set AppleScript's text item delimiters to "-"
    set dparts to text items of datePart
    set AppleScript's text item delimiters to ""
    if (count of dparts) is not 3 then return missing value
    set y to item 1 of dparts as integer
    set m to item 2 of dparts as integer
    set dy to item 3 of dparts as integer
    tell (current application)
        set d to current date
    end tell
    set year of d to y
    set month of d to m
    set day of d to dy
    if timePart is not "" then
        set AppleScript's text item delimiters to ":"
        set tparts to text items of timePart
        set AppleScript's text item delimiters to ""
        if (count of tparts) ≥ 2 then
            set h to item 1 of tparts as integer
            set mi to item 2 of tparts as integer
            set s to 0
            if (count of tparts) ≥ 3 then set s to item 3 of tparts as integer
            set time of d to (h * 3600 + mi * 60 + s)
        end if
    else
        set time of d to 0
    end if
    return d
end parse_date

on run argv
    set eventId to item 1 of argv
    set summaryText to item 2 of argv
    set startText to item 3 of argv
    set endText to item 4 of argv
    set alldayText to item 5 of argv
    set locationText to item 6 of argv
    set notesText to item 7 of argv
    set urlText to item 8 of argv
    set recurrenceText to item 9 of argv
    set statusText to item 10 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        set e to foundEvent
        if summaryText is not "" then set summary of e to summaryText
        if startText is not "" then
            set startDateVal to my parse_date(startText)
            if startDateVal is missing value then error "Invalid start date"
            set start date of e to startDateVal
        end if
        if endText is not "" then
            set endDateVal to my parse_date(endText)
            if endDateVal is missing value then error "Invalid end date"
            set end date of e to endDateVal
        end if
        if alldayText is "true" then set allday event of e to true
        if alldayText is "false" then set allday event of e to false
        if locationText is not "" then set location of e to locationText
        if notesText is not "" then set description of e to notesText
        if urlText is not "" then set url of e to urlText
        if recurrenceText is not "" then set recurrence of e to recurrenceText
        if statusText is "confirmed" then
            try
                set status of e to confirmed
            end try
        end if
        if statusText is "tentative" then
            try
                set status of e to tentative
            end try
        end if
        if statusText is "cancelled" then
            try
                set status of e to cancelled
            end try
        end if
        return (uid of e as string)
    end tell
end run
"#;
    let output = run_applescript(
        script,
        &[
            args.id,
            title,
            start,
            end,
            allday,
            location,
            notes,
            url,
            recurrence,
            status,
        ],
    )?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn calendar_delete(args: CalendarDeleteArgs) -> Result<()> {
    let script = r#"
on run argv
    set eventId to item 1 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        delete foundEvent
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn calendar_show(args: CalendarShowArgs) -> Result<()> {
    let script = r#"
on run argv
    set eventId to item 1 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        show foundEvent
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn calendar_alarms_list(args: CalendarAlarmsListArgs) -> Result<()> {
    let script = r#"
on run argv
    set eventId to item 1 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        set e to foundEvent
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        set idx to 1
        repeat with a in display alarms of e
            set trigInt to ""
            try
                set trigInt to (trigger interval of a as string)
            end try
            set trigDate to ""
            try
                set trigDate to (trigger date of a as string)
            end try
            set rec to "display" & fs & (idx as string) & fs & trigInt & fs & trigDate & fs & "" & fs & ""
            set end of outList to rec
            set idx to idx + 1
        end repeat
        set idx to 1
        repeat with a in mail alarms of e
            set trigInt to ""
            try
                set trigInt to (trigger interval of a as string)
            end try
            set trigDate to ""
            try
                set trigDate to (trigger date of a as string)
            end try
            set rec to "mail" & fs & (idx as string) & fs & trigInt & fs & trigDate & fs & "" & fs & ""
            set end of outList to rec
            set idx to idx + 1
        end repeat
        set idx to 1
        repeat with a in sound alarms of e
            set trigInt to ""
            try
                set trigInt to (trigger interval of a as string)
            end try
            set trigDate to ""
            try
                set trigDate to (trigger date of a as string)
            end try
            set sname to ""
            try
                set sname to (sound name of a as string)
            end try
            set sfile to ""
            try
                set sfile to (sound file of a as string)
            end try
            set rec to "sound" & fs & (idx as string) & fs & trigInt & fs & trigDate & fs & sname & fs & sfile
            set end of outList to rec
            set idx to idx + 1
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[args.id])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let alarm_type = r.get(0).cloned().unwrap_or_default();
            let index = r.get(1).cloned().unwrap_or_default();
            let trigger_interval = r.get(2).cloned().unwrap_or_default();
            let trigger_date = r.get(3).cloned().unwrap_or_default();
            let sound_name = r.get(4).cloned().unwrap_or_default();
            let sound_file = r.get(5).cloned().unwrap_or_default();
            json!({
                "type": alarm_type,
                "index": index,
                "trigger_interval": trigger_interval,
                "trigger_date": trigger_date,
                "sound_name": sound_name,
                "sound_file": sound_file
            })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn calendar_alarms_add(args: CalendarAlarmsAddArgs) -> Result<()> {
    if args.minutes.is_none() && args.date.is_none() {
        return Err(anyhow::anyhow!("provide --minutes or --date"));
    }
    let alarm_type = args.r#type;
    let minutes = args.minutes.map(|v| v.to_string()).unwrap_or_default();
    let date_text = args.date.unwrap_or_default();
    let sound_name = args.sound_name.unwrap_or_default();
    let sound_file = args.sound_file.unwrap_or_default();
    let script = r#"
use scripting additions
on parse_date(dateText)
    if dateText is "" then return missing value
    set datePart to dateText
    set timePart to ""
    if dateText contains " " then
        set AppleScript's text item delimiters to " "
        set parts to text items of dateText
        set AppleScript's text item delimiters to ""
        if (count of parts) ≥ 2 then
            set datePart to item 1 of parts
            set timePart to item 2 of parts
        end if
    end if
    set AppleScript's text item delimiters to "-"
    set dparts to text items of datePart
    set AppleScript's text item delimiters to ""
    if (count of dparts) is not 3 then return missing value
    set y to item 1 of dparts as integer
    set m to item 2 of dparts as integer
    set dy to item 3 of dparts as integer
    tell (current application)
        set d to current date
    end tell
    set year of d to y
    set month of d to m
    set day of d to dy
    if timePart is not "" then
        set AppleScript's text item delimiters to ":"
        set tparts to text items of timePart
        set AppleScript's text item delimiters to ""
        if (count of tparts) ≥ 2 then
            set h to item 1 of tparts as integer
            set mi to item 2 of tparts as integer
            set s to 0
            if (count of tparts) ≥ 3 then set s to item 3 of tparts as integer
            set time of d to (h * 3600 + mi * 60 + s)
        end if
    else
        set time of d to 0
    end if
    return d
end parse_date

on run argv
    set eventId to item 1 of argv
    set alarmType to item 2 of argv
    set minutesText to item 3 of argv
    set dateText to item 4 of argv
    set soundName to item 5 of argv
    set soundFile to item 6 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        set e to foundEvent
        set props to {}
        if minutesText is not "" then set props to props & {trigger interval:(minutesText as integer)}
        if dateText is not "" then
            set d to my parse_date(dateText)
            if d is missing value then error "Invalid date"
            set props to props & {trigger date:d}
        end if
        if alarmType is "display" then
            make new display alarm at end of display alarms of e with properties props
        else if alarmType is "mail" then
            make new mail alarm at end of mail alarms of e with properties props
        else if alarmType is "sound" then
            if soundName is not "" then set props to props & {sound name:soundName}
            if soundFile is not "" then set props to props & {sound file:soundFile}
            make new sound alarm at end of sound alarms of e with properties props
        else
            error "Unsupported alarm type"
        end if
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(
        script,
        &[
            args.id,
            alarm_type,
            minutes,
            date_text,
            sound_name,
            sound_file,
        ],
    )?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn calendar_alarms_delete(args: CalendarAlarmsDeleteArgs) -> Result<()> {
    let alarm_type = args.r#type;
    let index = args.index.to_string();
    let script = r#"
on run argv
    set eventId to item 1 of argv
    set alarmType to item 2 of argv
    set idxText to item 3 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        set e to foundEvent
        set idx to idxText as integer
        if alarmType is "display" then
            set a to display alarm idx of e
            delete a
        else if alarmType is "mail" then
            set a to mail alarm idx of e
            delete a
        else if alarmType is "sound" then
            set a to sound alarm idx of e
            delete a
        else
            error "Unsupported alarm type"
        end if
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id, alarm_type, index])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn calendar_attendees_list(args: CalendarAttendeesListArgs) -> Result<()> {
    let script = r#"
on run argv
    set eventId to item 1 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        set e to foundEvent
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with a in attendees of e
            set dn to ""
            try
                set dn to (display name of a as string)
            end try
            set em to ""
            try
                set em to (email of a as string)
            end try
            set ps to ""
            try
                set ps to (participation status of a as string)
            end try
            set rec to dn & fs & em & fs & ps
            set end of outList to rec
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[args.id])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let display_name = r.get(0).cloned().unwrap_or_default();
            let email = r.get(1).cloned().unwrap_or_default();
            let status = r.get(2).cloned().unwrap_or_default();
            json!({ "display_name": display_name, "email": email, "status": status })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn calendar_attendees_add(args: CalendarAttendeesAddArgs) -> Result<()> {
    let email = args.email;
    let script = r#"
on run argv
    set eventId to item 1 of argv
    set emailText to item 2 of argv
    tell application "Calendar"
        set foundEvent to missing value
        repeat with c in calendars
            set matches to (every event of c whose uid is eventId)
            if (count of matches) > 0 then
                set foundEvent to item 1 of matches
                exit repeat
            end if
        end repeat
        if foundEvent is missing value then error "Event not found: " & eventId
        set e to foundEvent
        make new attendee at end of attendees of e with properties {email:emailText}
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id, email])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}
