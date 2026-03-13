use anyhow::{anyhow, Result};
use serde_json::json;

use crate::common::{normalize_service_type, parse_records, run_applescript};
use crate::{
    MessagesBuddiesArgs, MessagesChatParticipantsArgs, MessagesChatsArgs, MessagesSendArgs,
    MessagesSendChatArgs,
};

pub fn messages_services() -> Result<()> {
    let script = r#"
on run argv
    tell application "/System/Applications/Messages.app"
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with a in accounts
            set svcType to ""
            try
                set svcType to (service type of a as string)
            on error
                try
                    set svcType to («property styp» of a as string)
                end try
            end try
            set rec to (id of a as string) & fs & (description of a as string) & fs & svcType
            set end of outList to rec
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let id = r.get(0).cloned().unwrap_or_default();
            let description = r.get(1).cloned().unwrap_or_default();
            let service_type = r.get(2).cloned().unwrap_or_default();
            json!({ "id": id, "description": description, "type": service_type })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn messages_buddies(args: MessagesBuddiesArgs) -> Result<()> {
    let service = args.service.unwrap_or_default();
    let service_type = args
        .r#type
        .map(|s| normalize_service_type(&s))
        .unwrap_or_default();
    let script = r#"
on run argv
    set serviceName to item 1 of argv
    set serviceTypeText to item 2 of argv
    tell application "/System/Applications/Messages.app"
        if serviceName is not "" then
            set targetAccount to first account whose description is serviceName
        else if serviceTypeText is "iMessage" then
            set targetAccount to first account whose service type is iMessage
        else if serviceTypeText is "SMS" then
            set targetAccount to first account whose service type is SMS
        else if serviceTypeText is "RCS" then
            set targetAccount to first account whose service type is RCS
        else
            set targetAccount to account 1
        end if
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        set seenHandles to {}
        try
            set targetChats to chats of targetAccount
        on error
            set targetChats to chats
        end try
        repeat with c in targetChats
            repeat with p in participants of c
                set h to ""
                try
                    set h to (handle of p as string)
                end try
                if h is not "" then
                    if h is not in seenHandles then
                        set end of seenHandles to h
                        set n to ""
                        try
                            set n to (name of p as string)
                        end try
                        set rec to n & fs & h
                        set end of outList to rec
                    end if
                end if
            end repeat
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[service, service_type])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let name = r.get(0).cloned().unwrap_or_default();
            let handle = r.get(1).cloned().unwrap_or_default();
            json!({ "name": name, "handle": handle })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn messages_send(args: MessagesSendArgs) -> Result<()> {
    if args.text.as_deref().unwrap_or("").is_empty()
        && args.file.as_deref().unwrap_or("").is_empty()
    {
        return Err(anyhow!("provide --text or --file"));
    }
    let service = args.service.unwrap_or_default();
    let service_type = args
        .r#type
        .map(|s| normalize_service_type(&s))
        .unwrap_or_default();
    let text = args.text.unwrap_or_default();
    let file = args.file.unwrap_or_default();
    let script = r#"
on run argv
    set serviceName to item 1 of argv
    set serviceTypeText to item 2 of argv
    set recipientHandle to item 3 of argv
    set messageText to item 4 of argv
    set filePath to item 5 of argv
    tell application "/System/Applications/Messages.app"
        if serviceName is not "" then
            set targetAccount to first account whose description is serviceName
        else if serviceTypeText is "iMessage" then
            set targetAccount to first account whose service type is iMessage
        else if serviceTypeText is "SMS" then
            set targetAccount to first account whose service type is SMS
        else
            set targetAccount to account 1
        end if
        set targetParticipant to missing value
        try
            set targetParticipant to participant recipientHandle of targetAccount
        on error
            try
                set targetParticipant to first participant of targetAccount whose handle is recipientHandle
            on error
                set targetParticipant to first participant of targetAccount whose name is recipientHandle
            end try
        end try
        if messageText is not "" then send messageText to targetParticipant
        if filePath is not "" then send POSIX file filePath to targetParticipant
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[service, service_type, args.to, text, file])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn messages_chats(args: MessagesChatsArgs) -> Result<()> {
    let service = args.service.unwrap_or_default();
    let service_type = args
        .r#type
        .map(|s| normalize_service_type(&s))
        .unwrap_or_default();
    let script = r#"
on run argv
    set serviceName to item 1 of argv
    set serviceTypeText to item 2 of argv
    tell application "/System/Applications/Messages.app"
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        set targetAccount to missing value
        if serviceName is not "" then
            set targetAccount to first account whose description is serviceName
        else if serviceTypeText is "iMessage" then
            set targetAccount to first account whose service type is iMessage
        else if serviceTypeText is "SMS" then
            set targetAccount to first account whose service type is SMS
        else if serviceTypeText is "RCS" then
            set targetAccount to first account whose service type is RCS
        end if
        if targetAccount is missing value then
            set targetChats to chats
        else
            set targetChats to chats of targetAccount
        end if
        repeat with c in targetChats
            set chatId to (id of c as string)
            set chatName to ""
            try
                set chatName to (name of c as string)
            end try
            set accountDesc to ""
            try
                set accountDesc to (description of (account of c) as string)
            end try
            set rec to chatId & fs & chatName & fs & accountDesc
            set end of outList to rec
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[service, service_type])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let id = r.get(0).cloned().unwrap_or_default();
            let name = r.get(1).cloned().unwrap_or_default();
            let account = r.get(2).cloned().unwrap_or_default();
            json!({ "id": id, "name": name, "account": account })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn messages_chat_participants(args: MessagesChatParticipantsArgs) -> Result<()> {
    let id = args.id.unwrap_or_default();
    let name = args.name.unwrap_or_default();
    if id.is_empty() && name.is_empty() {
        return Err(anyhow!("provide --id or --name"));
    }
    let script = r#"
on run argv
    set chatId to item 1 of argv
    set chatName to item 2 of argv
    tell application "/System/Applications/Messages.app"
        if chatId is not "" then
            set matches to (every chat whose id is chatId)
        else
            set matches to (every chat whose name is chatName)
        end if
        if (count of matches) is 0 then error "Chat not found"
        set c to item 1 of matches
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with p in participants of c
            set n to ""
            try
                set n to (name of p as string)
            end try
            set h to ""
            try
                set h to (handle of p as string)
            end try
            set rec to n & fs & h
            set end of outList to rec
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[id, name])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let name = r.get(0).cloned().unwrap_or_default();
            let handle = r.get(1).cloned().unwrap_or_default();
            json!({ "name": name, "handle": handle })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn messages_send_chat(args: MessagesSendChatArgs) -> Result<()> {
    if args.text.as_deref().unwrap_or("").is_empty()
        && args.file.as_deref().unwrap_or("").is_empty()
    {
        return Err(anyhow!("provide --text or --file"));
    }
    let id = args.id.unwrap_or_default();
    let name = args.name.unwrap_or_default();
    if id.is_empty() && name.is_empty() {
        return Err(anyhow!("provide --id or --name"));
    }
    let text = args.text.unwrap_or_default();
    let file = args.file.unwrap_or_default();
    let script = r#"
on run argv
    set chatId to item 1 of argv
    set chatName to item 2 of argv
    set messageText to item 3 of argv
    set filePath to item 4 of argv
    tell application "/System/Applications/Messages.app"
        if chatId is not "" then
            set matches to (every chat whose id is chatId)
        else
            set matches to (every chat whose name is chatName)
        end if
        if (count of matches) is 0 then error "Chat not found"
        set c to item 1 of matches
        if messageText is not "" then send messageText to c
        if filePath is not "" then send POSIX file filePath to c
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[id, name, text, file])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}
