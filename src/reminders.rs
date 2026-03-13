use anyhow::Result;
use serde_json::json;

use crate::common::{parse_records, run_applescript, FS, RS};
use crate::{
    RemindersCompleteArgs, RemindersCreateArgs, RemindersDeleteArgs, RemindersGetArgs,
    RemindersListArgs, RemindersListCreateArgs, RemindersListDeleteArgs, RemindersListUpdateArgs,
    RemindersUpdateArgs,
};

pub fn reminders_lists() -> Result<()> {
    let script = r#"
on run argv
    tell application "/System/Applications/Reminders.app"
        set rs to character id 30
        set outList to {}
        repeat with l in lists
            set end of outList to (name of l as string)
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

pub fn reminders_lists_create(args: RemindersListCreateArgs) -> Result<()> {
    let account = args.account.unwrap_or_default();
    let name = args.name;
    let color = args.color.unwrap_or_default();
    let emblem = args.emblem.unwrap_or_default();
    let script = r#"
on run argv
    set accountName to item 1 of argv
    set listName to item 2 of argv
    set colorText to item 3 of argv
    set emblemText to item 4 of argv
    tell application "/System/Applications/Reminders.app"
        if accountName is "" then
            set targetAccount to account 1
        else
            if not (exists account accountName) then error "Account not found: " & accountName
            set targetAccount to account accountName
        end if
        if (exists list listName) then error "List already exists: " & listName
        set props to {name:listName}
        if colorText is not "" then set props to props & {color:colorText}
        if emblemText is not "" then set props to props & {emblem:emblemText}
        set newList to make new list at targetAccount with properties props
        return (id of newList as string)
    end tell
end run
"#;
    let output = run_applescript(script, &[account, name, color, emblem])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn reminders_lists_update(args: RemindersListUpdateArgs) -> Result<()> {
    let name = args.name;
    let new_name = args.new_name.unwrap_or_default();
    let color = args.color.unwrap_or_default();
    let emblem = args.emblem.unwrap_or_default();
    let script = r#"
on run argv
    set listName to item 1 of argv
    set newName to item 2 of argv
    set colorText to item 3 of argv
    set emblemText to item 4 of argv
    tell application "/System/Applications/Reminders.app"
        if not (exists list listName) then error "List not found: " & listName
        set l to list listName
        if newName is not "" then set name of l to newName
        if colorText is not "" then set color of l to colorText
        if emblemText is not "" then set emblem of l to emblemText
        return (id of l as string)
    end tell
end run
"#;
    let output = run_applescript(script, &[name, new_name, color, emblem])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn reminders_lists_delete(args: RemindersListDeleteArgs) -> Result<()> {
    let name = args.name;
    let script = r#"
on run argv
    set listName to item 1 of argv
    tell application "/System/Applications/Reminders.app"
        if not (exists list listName) then error "List not found: " & listName
        delete list listName
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[name])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn reminders_list(args: RemindersListArgs) -> Result<()> {
    let list = args.list.unwrap_or_default();
    let limit = args.limit.to_string();
    let completed = args
        .completed
        .map(|v| if v { "true" } else { "false" }.to_string())
        .unwrap_or_default();
    let script = r#"
on run argv
    set listName to item 1 of argv
    set limitText to item 2 of argv
    set completedText to item 3 of argv
    if limitText is "" then
        set maxCount to 0
    else
        set maxCount to limitText as integer
    end if
    tell application "/System/Applications/Reminders.app"
        if listName is "" then
            set targetList to list 1
        else
            if not (exists list listName) then error "List not found: " & listName
            set targetList to list listName
        end if
        if completedText is "" then
            set matches to (every reminder of targetList)
        else if completedText is "true" then
            set matches to (every reminder of targetList whose completed is true)
        else
            set matches to (every reminder of targetList whose completed is false)
        end if
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with r in matches
            set dueText to ""
            try
                if due date of r is not missing value then set dueText to (due date of r as string)
            end try
            set prioText to ""
            try
                set prioText to (priority of r as string)
            end try
            set rec to (id of r as string) & fs & (name of r as string) & fs & (completed of r as string) & fs & dueText & fs & prioText
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
    let output = run_applescript(script, &[list, limit, completed])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let id = r.get(0).cloned().unwrap_or_default();
            let name = r.get(1).cloned().unwrap_or_default();
            let completed = r.get(2).cloned().unwrap_or_default();
            let due = r.get(3).cloned().unwrap_or_default();
            let priority = r.get(4).cloned().unwrap_or_default();
            json!({ "id": id, "name": name, "completed": completed, "due": due, "priority": priority })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn reminders_get(args: RemindersGetArgs) -> Result<()> {
    let script = r#"
on run argv
    set reminderId to item 1 of argv
    tell application "/System/Applications/Reminders.app"
        if not (exists reminder id reminderId) then error "Reminder not found: " & reminderId
        set r to reminder id reminderId
        set fs to character id 31
        set dueText to ""
        try
            if due date of r is not missing value then set dueText to (due date of r as string)
        end try
        set prioText to ""
        try
            set prioText to (priority of r as string)
        end try
        set bodyText to ""
        try
            set bodyText to (body of r as string)
        end try
        set listName to ""
        try
            set listName to (name of (container of r) as string)
        end try
        set createdText to ""
        try
            if creation date of r is not missing value then set createdText to (creation date of r as string)
        end try
        set modifiedText to ""
        try
            if modification date of r is not missing value then set modifiedText to (modification date of r as string)
        end try
        set completedAtText to ""
        try
            if completion date of r is not missing value then set completedAtText to (completion date of r as string)
        end try
        set allDayDueText to ""
        try
            if allday due date of r is not missing value then set allDayDueText to (allday due date of r as string)
        end try
        set remindText to ""
        try
            if remind me date of r is not missing value then set remindText to (remind me date of r as string)
        end try
        set flaggedText to ""
        try
            set flaggedText to (flagged of r as string)
        end try
        return (id of r as string) & fs & (name of r as string) & fs & listName & fs & (completed of r as string) & fs & dueText & fs & prioText & fs & bodyText & fs & createdText & fs & modifiedText & fs & completedAtText & fs & allDayDueText & fs & remindText & fs & flaggedText
    end tell
end run
"#;
    let output = run_applescript(script, &[args.id])?;
    let fields: Vec<String> = output.split(FS).map(|f| f.to_string()).collect();
    let id = fields.get(0).cloned().unwrap_or_default();
    let name = fields.get(1).cloned().unwrap_or_default();
    let list = fields.get(2).cloned().unwrap_or_default();
    let completed = fields.get(3).cloned().unwrap_or_default();
    let due = fields.get(4).cloned().unwrap_or_default();
    let priority = fields.get(5).cloned().unwrap_or_default();
    let body = fields.get(6).cloned().unwrap_or_default();
    let created_at = fields.get(7).cloned().unwrap_or_default();
    let modified_at = fields.get(8).cloned().unwrap_or_default();
    let completed_at = fields.get(9).cloned().unwrap_or_default();
    let allday_due = fields.get(10).cloned().unwrap_or_default();
    let remind_me = fields.get(11).cloned().unwrap_or_default();
    let flagged = fields.get(12).cloned().unwrap_or_default();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "id": id,
            "name": name,
            "list": list,
            "completed": completed,
            "due": due,
            "priority": priority,
            "body": body,
            "created_at": created_at,
            "modified_at": modified_at,
            "completed_at": completed_at,
            "allday_due": allday_due,
            "remind_me": remind_me,
            "flagged": flagged
        }))?
    );
    Ok(())
}

pub fn reminders_create(args: RemindersCreateArgs) -> Result<()> {
    let list = args.list.unwrap_or_default();
    let parent = args.parent.unwrap_or_default();
    let title = args.title;
    let body = args.body.unwrap_or_default();
    let due = args.due.unwrap_or_default();
    let allday_due = args.allday_due.unwrap_or_default();
    let remind_me = args.remind_me.unwrap_or_default();
    let priority = args.priority.map(|v| v.to_string()).unwrap_or_default();
    let flagged = args
        .flagged
        .map(|v| if v { "true" } else { "false" }.to_string())
        .unwrap_or_default();
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
    set listName to item 1 of argv
    set parentId to item 2 of argv
    set reminderTitle to item 3 of argv
    set reminderBody to item 4 of argv
    set dueText to item 5 of argv
    set allDayDueText to item 6 of argv
    set remindText to item 7 of argv
    set priorityText to item 8 of argv
    set flaggedText to item 9 of argv
    tell application "/System/Applications/Reminders.app"
        set props to {name:reminderTitle}
        if reminderBody is not "" then set props to props & {body:reminderBody}
        set dueDateVal to my parse_date(dueText)
        if dueDateVal is not missing value then set props to props & {due date:dueDateVal}
        set allDayDueVal to my parse_date(allDayDueText)
        if allDayDueVal is not missing value then set props to props & {allday due date:allDayDueVal}
        set remindVal to my parse_date(remindText)
        if remindVal is not missing value then set props to props & {remind me date:remindVal}
        if priorityText is not "" then set props to props & {priority:(priorityText as integer)}
        if flaggedText is "true" then set props to props & {flagged:true}
        if flaggedText is "false" then set props to props & {flagged:false}
        if parentId is not "" then
            if not (exists reminder id parentId) then error "Parent reminder not found: " & parentId
            set newReminder to make new reminder at (reminder id parentId) with properties props
        else
            if listName is "" then
                set targetList to list 1
            else
                if not (exists list listName) then error "List not found: " & listName
                set targetList to list listName
            end if
            set newReminder to make new reminder at targetList with properties props
        end if
        return (id of newReminder as string)
    end tell
end run
"#;
    let output = run_applescript(
        script,
        &[
            list, parent, title, body, due, allday_due, remind_me, priority, flagged,
        ],
    )?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn reminders_update(args: RemindersUpdateArgs) -> Result<()> {
    let title = args.title.unwrap_or_default();
    let body = args.body.unwrap_or_default();
    let due = args.due.unwrap_or_default();
    let allday_due = args.allday_due.unwrap_or_default();
    let remind_me = args.remind_me.unwrap_or_default();
    let priority = args.priority.map(|v| v.to_string()).unwrap_or_default();
    let completed = args
        .completed
        .map(|v| if v { "true" } else { "false" }.to_string())
        .unwrap_or_default();
    let flagged = args
        .flagged
        .map(|v| if v { "true" } else { "false" }.to_string())
        .unwrap_or_default();
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
    set reminderId to item 1 of argv
    set reminderTitle to item 2 of argv
    set reminderBody to item 3 of argv
    set dueText to item 4 of argv
    set allDayDueText to item 5 of argv
    set remindText to item 6 of argv
    set priorityText to item 7 of argv
    set completedText to item 8 of argv
    set flaggedText to item 9 of argv
    tell application "/System/Applications/Reminders.app"
        if not (exists reminder id reminderId) then error "Reminder not found: " & reminderId
        set r to reminder id reminderId
        if reminderTitle is not "" then set name of r to reminderTitle
        if reminderBody is not "" then set body of r to reminderBody
        set dueDateVal to my parse_date(dueText)
        if dueDateVal is not missing value then set due date of r to dueDateVal
        set allDayDueVal to my parse_date(allDayDueText)
        if allDayDueVal is not missing value then set allday due date of r to allDayDueVal
        set remindVal to my parse_date(remindText)
        if remindVal is not missing value then set remind me date of r to remindVal
        if priorityText is not "" then set priority of r to (priorityText as integer)
        if completedText is "true" then set completed of r to true
        if completedText is "false" then set completed of r to false
        if flaggedText is "true" then set flagged of r to true
        if flaggedText is "false" then set flagged of r to false
        return (id of r as string)
    end tell
end run
"#;
    let output = run_applescript(
        script,
        &[
            args.id,
            title,
            body,
            due,
            allday_due,
            remind_me,
            priority,
            completed,
            flagged,
        ],
    )?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn reminders_complete(args: RemindersCompleteArgs) -> Result<()> {
    let script = r#"
on run argv
    set reminderId to item 1 of argv
    tell application "/System/Applications/Reminders.app"
        if not (exists reminder id reminderId) then error "Reminder not found: " & reminderId
        set completed of (reminder id reminderId) to true
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn reminders_delete(args: RemindersDeleteArgs) -> Result<()> {
    let script = r#"
on run argv
    set reminderId to item 1 of argv
    tell application "Reminders"
        if not (exists reminder id reminderId) then error "Reminder not found: " & reminderId
        delete reminder id reminderId
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}
