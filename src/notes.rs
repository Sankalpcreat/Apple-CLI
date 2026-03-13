use anyhow::Result;
use serde_json::json;

use crate::common::{parse_records, run_applescript, FS, RS};
use crate::{
    FoldersCreateArgs, FoldersDeleteArgs, FoldersListArgs, NotesAttachmentsDeleteArgs,
    NotesAttachmentsListArgs, NotesAttachmentsSaveArgs, NotesCreateArgs, NotesDeleteArgs,
    NotesGetArgs, NotesListArgs, NotesMoveArgs, NotesSearchArgs, NotesShowArgs, NotesUpdateArgs,
};

pub fn accounts_list() -> Result<()> {
    let script = r#"
on run argv
    tell application "Notes"
        set rs to character id 30
        set outList to {}
        repeat with a in accounts
            set end of outList to (name of a as string)
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

pub fn folders_list(args: FoldersListArgs) -> Result<()> {
    let account = args.account.unwrap_or_default();
    let script = r#"
on run argv
    set accountName to item 1 of argv
    tell application "Notes"
        if accountName is "" then
            set targetAccount to account 1
        else
            if not (exists account accountName) then error "Account not found: " & accountName
            set targetAccount to account accountName
        end if
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with f in folders of targetAccount
            set rec to (id of f as string) & fs & (name of f as string)
            set end of outList to rec
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[account])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let id = r.get(0).cloned().unwrap_or_default();
            let name = r.get(1).cloned().unwrap_or_default();
            json!({ "id": id, "name": name })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn folders_create(args: FoldersCreateArgs) -> Result<()> {
    let account = args.account.unwrap_or_default();
    let name = args.name;
    let parent = args.parent.unwrap_or_default();
    let script = r#"
on run argv
    set accountName to item 1 of argv
    set folderName to item 2 of argv
    set parentName to item 3 of argv
    tell application "Notes"
        if accountName is "" then
            set targetAccount to account 1
        else
            if not (exists account accountName) then error "Account not found: " & accountName
            set targetAccount to account accountName
        end if
        if parentName is "" then
            if not (exists folder folderName of targetAccount) then
                make new folder at targetAccount with properties {name:folderName}
            end if
        else
            if not (exists folder parentName of targetAccount) then error "Parent folder not found: " & parentName
            tell folder parentName of targetAccount
                if not (exists folder folderName) then
                    make new folder with properties {name:folderName}
                end if
            end tell
        end if
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[account, name, parent])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn folders_delete(args: FoldersDeleteArgs) -> Result<()> {
    let account = args.account.unwrap_or_default();
    let name = args.name;
    let parent = args.parent.unwrap_or_default();
    let script = r#"
on run argv
    set accountName to item 1 of argv
    set folderName to item 2 of argv
    set parentName to item 3 of argv
    tell application "Notes"
        if accountName is "" then
            set targetAccount to account 1
        else
            if not (exists account accountName) then error "Account not found: " & accountName
            set targetAccount to account accountName
        end if
        if parentName is "" then
            if not (exists folder folderName of targetAccount) then error "Folder not found: " & folderName
            delete folder folderName of targetAccount
        else
            if not (exists folder parentName of targetAccount) then error "Parent folder not found: " & parentName
            if not (exists folder folderName of folder parentName of targetAccount) then error "Folder not found: " & folderName
            delete folder folderName of folder parentName of targetAccount
        end if
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[account, name, parent])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn notes_list(args: NotesListArgs) -> Result<()> {
    let account = args.account.unwrap_or_default();
    let folder = args.folder.unwrap_or_default();
    let script = r#"
on run argv
    set accountName to item 1 of argv
    set folderName to item 2 of argv
    tell application "Notes"
        if accountName is "" then
            set targetAccount to account 1
        else
            if not (exists account accountName) then error "Account not found: " & accountName
            set targetAccount to account accountName
        end if
        if folderName is "" then
            set targetNotes to every note of targetAccount
        else
            if not (exists folder folderName of targetAccount) then error "Folder not found: " & folderName
            set targetNotes to every note of folder folderName of targetAccount
        end if
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with n in targetNotes
            set folderName to ""
            try
                set folderName to (name of container of n as string)
            end try
            set createdText to ""
            try
                set createdText to (creation date of n as string)
            end try
            set modifiedText to ""
            try
                set modifiedText to (modification date of n as string)
            end try
            set protectedText to ""
            try
                set protectedText to (password protected of n as string)
            end try
            set sharedText to ""
            try
                set sharedText to (shared of n as string)
            end try
            set rec to (id of n as string) & fs & (name of n as string) & fs & folderName & fs & createdText & fs & modifiedText & fs & protectedText & fs & sharedText
            set end of outList to rec
        end repeat
        set AppleScript's text item delimiters to rs
        set outText to outList as text
        set AppleScript's text item delimiters to ""
        return outText
    end tell
end run
"#;
    let output = run_applescript(script, &[account, folder])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let id = r.get(0).cloned().unwrap_or_default();
            let name = r.get(1).cloned().unwrap_or_default();
            let folder = r.get(2).cloned().unwrap_or_default();
            let created_at = r.get(3).cloned().unwrap_or_default();
            let modified_at = r.get(4).cloned().unwrap_or_default();
            let password_protected = r.get(5).cloned().unwrap_or_default();
            let shared = r.get(6).cloned().unwrap_or_default();
            json!({
                "id": id,
                "name": name,
                "folder": folder,
                "created_at": created_at,
                "modified_at": modified_at,
                "password_protected": password_protected,
                "shared": shared
            })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn notes_get(args: NotesGetArgs) -> Result<()> {
    let script = r#"
on run argv
    set noteId to item 1 of argv
    tell application "Notes"
        if not (exists note id noteId) then error "Note not found: " & noteId
        set n to note id noteId
        set fs to character id 31
        set folderName to ""
        try
            set folderName to (name of container of n as string)
        end try
        set createdText to ""
        try
            set createdText to (creation date of n as string)
        end try
        set modifiedText to ""
        try
            set modifiedText to (modification date of n as string)
        end try
        set protectedText to ""
        try
            set protectedText to (password protected of n as string)
        end try
        set sharedText to ""
        try
            set sharedText to (shared of n as string)
        end try
        set plainText to ""
        try
            set plainText to (plaintext of n as string)
        end try
        return (id of n as string) & fs & (name of n as string) & fs & folderName & fs & (body of n as string) & fs & plainText & fs & createdText & fs & modifiedText & fs & protectedText & fs & sharedText
    end tell
end run
"#;
    let output = run_applescript(script, &[args.id])?;
    let fields: Vec<String> = output.split(FS).map(|f| f.to_string()).collect();
    let id = fields.get(0).cloned().unwrap_or_default();
    let name = fields.get(1).cloned().unwrap_or_default();
    let folder = fields.get(2).cloned().unwrap_or_default();
    let body = fields.get(3).cloned().unwrap_or_default();
    let plaintext = fields.get(4).cloned().unwrap_or_default();
    let created_at = fields.get(5).cloned().unwrap_or_default();
    let modified_at = fields.get(6).cloned().unwrap_or_default();
    let password_protected = fields.get(7).cloned().unwrap_or_default();
    let shared = fields.get(8).cloned().unwrap_or_default();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "id": id,
            "name": name,
            "folder": folder,
            "body": body,
            "plaintext": plaintext,
            "created_at": created_at,
            "modified_at": modified_at,
            "password_protected": password_protected,
            "shared": shared
        }))?
    );
    Ok(())
}

pub fn notes_create(args: NotesCreateArgs) -> Result<()> {
    let account = args.account.unwrap_or_default();
    let folder = args.folder.unwrap_or_else(|| "Notes".to_string());
    let name = args.name.unwrap_or_else(|| "Untitled".to_string());
    let body = args.body;
    let attach = args.attach;
    let script = r#"
on run argv
    set accountName to item 1 of argv
    set folderName to item 2 of argv
    set noteName to item 3 of argv
    set noteBody to item 4 of argv
    set attachText to item 5 of argv
    tell application "Notes"
        if accountName is "" then
            set targetAccount to account 1
        else
            if not (exists account accountName) then error "Account not found: " & accountName
            set targetAccount to account accountName
        end if
        if not (exists folder folderName of targetAccount) then error "Folder not found: " & folderName
        set newNote to make new note at folder folderName of targetAccount with properties {name:noteName, body:noteBody}
        if attachText is not "" then
            set AppleScript's text item delimiters to linefeed
            set fileList to text items of attachText
            set AppleScript's text item delimiters to ""
            repeat with fp in fileList
                if fp is not "" then
                    make new attachment at end of attachments of newNote with data (POSIX file fp)
                end if
            end repeat
        end if
        return (id of newNote as string)
    end tell
end run
"#;
    let attach_blob = if attach.is_empty() {
        "".to_string()
    } else {
        attach.join("\n")
    };
    let output = run_applescript(script, &[account, folder, name, body, attach_blob])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn notes_update(args: NotesUpdateArgs) -> Result<()> {
    let name = args.name.unwrap_or_default();
    let body = args.body.unwrap_or_default();
    let attach = args.attach;
    let script = r#"
on run argv
    set noteId to item 1 of argv
    set noteName to item 2 of argv
    set noteBody to item 3 of argv
    set attachText to item 4 of argv
    tell application "Notes"
        if not (exists note id noteId) then error "Note not found: " & noteId
        set n to note id noteId
        if noteName is not "" then set name of n to noteName
        if noteBody is not "" then set body of n to noteBody
        if attachText is not "" then
            set AppleScript's text item delimiters to linefeed
            set fileList to text items of attachText
            set AppleScript's text item delimiters to ""
            repeat with fp in fileList
                if fp is not "" then
                    make new attachment at end of attachments of n with data (POSIX file fp)
                end if
            end repeat
        end if
        return (id of n as string)
    end tell
end run
"#;
    let attach_blob = if attach.is_empty() {
        "".to_string()
    } else {
        attach.join("\n")
    };
    let output = run_applescript(script, &[args.id, name, body, attach_blob])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "id": output }))?);
    Ok(())
}

pub fn notes_delete(args: NotesDeleteArgs) -> Result<()> {
    let script = r#"
on run argv
    set noteId to item 1 of argv
    tell application "Notes"
        if not (exists note id noteId) then error "Note not found: " & noteId
        delete note id noteId
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn notes_move(args: NotesMoveArgs) -> Result<()> {
    let account = args.account.unwrap_or_default();
    let folder = args.folder;
    let script = r#"
on run argv
    set noteId to item 1 of argv
    set accountName to item 2 of argv
    set folderName to item 3 of argv
    tell application "Notes"
        if not (exists note id noteId) then error "Note not found: " & noteId
        if accountName is "" then
            set targetAccount to account 1
        else
            if not (exists account accountName) then error "Account not found: " & accountName
            set targetAccount to account accountName
        end if
        if not (exists folder folderName of targetAccount) then error "Folder not found: " & folderName
        move note id noteId to folder folderName of targetAccount
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id, account, folder])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn notes_search(args: NotesSearchArgs) -> Result<()> {
    let account = args.account.unwrap_or_default();
    let query = args.query;
    let limit = args.limit;
    let script = r#"
on run argv
    set accountName to item 1 of argv
    set queryText to item 2 of argv
    set limitText to item 3 of argv
    if limitText is "" then
        set maxCount to 0
    else
        set maxCount to limitText as integer
    end if
    tell application "Notes"
        if accountName is "" then
            set targetAccount to account 1
        else
            if not (exists account accountName) then error "Account not found: " & accountName
            set targetAccount to account accountName
        end if
        set matches to (every note of targetAccount whose name contains queryText or body contains queryText)
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with n in matches
            set folderName to ""
            try
                set folderName to (name of container of n as string)
            end try
            set rec to (id of n as string) & fs & (name of n as string) & fs & folderName
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
    let output = run_applescript(script, &[account, query, limit.to_string()])?;
    let records = parse_records(&output);
    let items: Vec<_> = records
        .into_iter()
        .map(|r| {
            let id = r.get(0).cloned().unwrap_or_default();
            let name = r.get(1).cloned().unwrap_or_default();
            let folder = r.get(2).cloned().unwrap_or_default();
            json!({ "id": id, "name": name, "folder": folder })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn notes_show(args: NotesShowArgs) -> Result<()> {
    let script = r#"
on run argv
    set noteId to item 1 of argv
    tell application "Notes"
        if not (exists note id noteId) then error "Note not found: " & noteId
        show note id noteId
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}

pub fn notes_attachments_list(args: NotesAttachmentsListArgs) -> Result<()> {
    let script = r#"
on run argv
    set noteId to item 1 of argv
    tell application "Notes"
        if not (exists note id noteId) then error "Note not found: " & noteId
        set n to note id noteId
        set fs to character id 31
        set rs to character id 30
        set outList to {}
        repeat with a in attachments of n
            set cidText to ""
            try
                set cidText to (content identifier of a as string)
            end try
            set createdText to ""
            try
                set createdText to (creation date of a as string)
            end try
            set modifiedText to ""
            try
                set modifiedText to (modification date of a as string)
            end try
            set urlText to ""
            try
                set urlText to (URL of a as string)
            end try
            set sharedText to ""
            try
                set sharedText to (shared of a as string)
            end try
            set rec to (id of a as string) & fs & (name of a as string) & fs & cidText & fs & createdText & fs & modifiedText & fs & urlText & fs & sharedText
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
            let id = r.get(0).cloned().unwrap_or_default();
            let name = r.get(1).cloned().unwrap_or_default();
            let content_id = r.get(2).cloned().unwrap_or_default();
            let created_at = r.get(3).cloned().unwrap_or_default();
            let modified_at = r.get(4).cloned().unwrap_or_default();
            let url = r.get(5).cloned().unwrap_or_default();
            let shared = r.get(6).cloned().unwrap_or_default();
            json!({
                "id": id,
                "name": name,
                "content_identifier": content_id,
                "created_at": created_at,
                "modified_at": modified_at,
                "url": url,
                "shared": shared
            })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&items)?);
    Ok(())
}

pub fn notes_attachments_save(args: NotesAttachmentsSaveArgs) -> Result<()> {
    if args.attachment_id.is_none() && args.name.is_none() {
        return Err(anyhow::anyhow!("provide --attachment-id or --name"));
    }
    let att_id = args.attachment_id.unwrap_or_default();
    let name = args.name.unwrap_or_default();
    let output_dir = args.output;
    let script = r#"
on run argv
    set noteId to item 1 of argv
    set attId to item 2 of argv
    set attName to item 3 of argv
    set outDir to item 4 of argv
    tell application "Notes"
        if not (exists note id noteId) then error "Note not found: " & noteId
        set n to note id noteId
        set target to missing value
        if attId is not "" then
            set target to first attachment of n whose id is attId
        else
            set target to first attachment of n whose name is attName
        end if
        if target is missing value then error "Attachment not found"
        set outDirAlias to POSIX file outDir as alias
        set outDirPosix to POSIX path of outDirAlias
        set outFilePosix to outDirPosix & (name of target as string)
        set outFileHfs to (outDirAlias as text) & (name of target as string)
        save target in file outFileHfs
        return outFilePosix
    end tell
end run
"#;
    let output = run_applescript(script, &[args.id, att_id, name, output_dir])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "path": output }))?);
    Ok(())
}

pub fn notes_attachments_delete(args: NotesAttachmentsDeleteArgs) -> Result<()> {
    if args.attachment_id.is_none() && args.name.is_none() {
        return Err(anyhow::anyhow!("provide --attachment-id or --name"));
    }
    let att_id = args.attachment_id.unwrap_or_default();
    let name = args.name.unwrap_or_default();
    let script = r#"
on run argv
    set noteId to item 1 of argv
    set attId to item 2 of argv
    set attName to item 3 of argv
    tell application "Notes"
        if not (exists note id noteId) then error "Note not found: " & noteId
        set n to note id noteId
        set target to missing value
        if attId is not "" then
            set target to first attachment of n whose id is attId
        else
            set target to first attachment of n whose name is attName
        end if
        if target is missing value then error "Attachment not found"
        delete target
        return "OK"
    end tell
end run
"#;
    let _ = run_applescript(script, &[args.id, att_id, name])?;
    println!("{}", serde_json::to_string_pretty(&json!({ "status": "OK" }))?);
    Ok(())
}
