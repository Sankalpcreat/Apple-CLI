use anyhow::Result;
use clap::{Args, Parser, Subcommand};
mod calendar;
mod common;
mod messages;
mod notes;
mod reminders;

#[derive(Parser)]
#[command(name = "apple", version, about = "Apple CLI for macOS (Notes/Reminders/Calendar/Messages)")]
struct Cli {
    #[command(subcommand)]
    command: TopCommand,
}

#[derive(Subcommand)]
enum TopCommand {
    Notes(NotesCmd),
    Reminders(RemindersCmd),
    Calendar(CalendarCmd),
    Messages(MessagesCmd),
}

#[derive(Args)]
struct AccountsCmd {
    #[command(subcommand)]
    command: AccountsSubcommand,
}

#[derive(Subcommand)]
enum AccountsSubcommand {
    List,
}

#[derive(Args)]
struct FoldersCmd {
    #[command(subcommand)]
    command: FoldersSubcommand,
}

#[derive(Subcommand)]
enum FoldersSubcommand {
    List(FoldersListArgs),
    Create(FoldersCreateArgs),
    Delete(FoldersDeleteArgs),
}

#[derive(Args)]
struct NotesCmd {
    #[command(subcommand)]
    command: NotesSubcommand,
}

#[derive(Subcommand)]
enum NotesSubcommand {
    Accounts(AccountsCmd),
    Folders(FoldersCmd),
    List(NotesListArgs),
    Get(NotesGetArgs),
    Create(NotesCreateArgs),
    Update(NotesUpdateArgs),
    Delete(NotesDeleteArgs),
    Move(NotesMoveArgs),
    Search(NotesSearchArgs),
    Show(NotesShowArgs),
    Attachments(NotesAttachmentsCmd),
}

#[derive(Args)]
struct RemindersCmd {
    #[command(subcommand)]
    command: RemindersSubcommand,
}

#[derive(Subcommand)]
enum RemindersSubcommand {
    Lists,
    ListsCreate(RemindersListCreateArgs),
    ListsUpdate(RemindersListUpdateArgs),
    ListsDelete(RemindersListDeleteArgs),
    List(RemindersListArgs),
    Get(RemindersGetArgs),
    Create(RemindersCreateArgs),
    Update(RemindersUpdateArgs),
    Complete(RemindersCompleteArgs),
    Delete(RemindersDeleteArgs),
}

#[derive(Args)]
struct CalendarCmd {
    #[command(subcommand)]
    command: CalendarSubcommand,
}

#[derive(Subcommand)]
enum CalendarSubcommand {
    Calendars,
    CalendarsCreate(CalendarCalendarsCreateArgs),
    CalendarsDelete(CalendarCalendarsDeleteArgs),
    Events(CalendarEventsArgs),
    Get(CalendarGetArgs),
    Create(CalendarCreateArgs),
    Update(CalendarUpdateArgs),
    Delete(CalendarDeleteArgs),
    Show(CalendarShowArgs),
    Alarms(CalendarAlarmsCmd),
    Attendees(CalendarAttendeesCmd),
}

#[derive(Args)]
struct MessagesCmd {
    #[command(subcommand)]
    command: MessagesSubcommand,
}

#[derive(Subcommand)]
enum MessagesSubcommand {
    Services,
    Buddies(MessagesBuddiesArgs),
    Chats(MessagesChatsArgs),
    ChatParticipants(MessagesChatParticipantsArgs),
    Send(MessagesSendArgs),
    SendChat(MessagesSendChatArgs),
}

#[derive(Args)]
struct FoldersListArgs {
    /// Target account name (default: first account)
    #[arg(long)]
    account: Option<String>,
}

#[derive(Args)]
struct FoldersCreateArgs {
    /// Target account name (default: first account)
    #[arg(long)]
    account: Option<String>,
    /// Folder name to create
    #[arg(long)]
    name: String,
    /// Optional parent folder name
    #[arg(long)]
    parent: Option<String>,
}

#[derive(Args)]
struct FoldersDeleteArgs {
    /// Target account name (default: first account)
    #[arg(long)]
    account: Option<String>,
    /// Folder name to delete
    #[arg(long)]
    name: String,
    /// Optional parent folder name
    #[arg(long)]
    parent: Option<String>,
}

#[derive(Args)]
struct NotesListArgs {
    /// Target account name (default: first account)
    #[arg(long)]
    account: Option<String>,
    /// Optional folder name filter
    #[arg(long)]
    folder: Option<String>,
}

#[derive(Args)]
struct NotesGetArgs {
    /// Note id
    id: String,
}

#[derive(Args)]
struct NotesCreateArgs {
    /// Target account name (default: first account)
    #[arg(long)]
    account: Option<String>,
    /// Folder name (default: "Notes")
    #[arg(long)]
    folder: Option<String>,
    /// Note title (default: "Untitled")
    #[arg(long)]
    name: Option<String>,
    /// Note body (HTML or plain text)
    #[arg(long)]
    body: String,
    /// File(s) to attach
    #[arg(long, num_args = 1..)]
    attach: Vec<String>,
}

#[derive(Args)]
struct NotesUpdateArgs {
    /// Note id
    id: String,
    /// New title
    #[arg(long)]
    name: Option<String>,
    /// New body (HTML or plain text)
    #[arg(long)]
    body: Option<String>,
    /// File(s) to attach
    #[arg(long, num_args = 1..)]
    attach: Vec<String>,
}

#[derive(Args)]
struct NotesDeleteArgs {
    /// Note id
    id: String,
}

#[derive(Args)]
struct NotesMoveArgs {
    /// Note id
    id: String,
    /// Target account name (default: first account)
    #[arg(long)]
    account: Option<String>,
    /// Target folder name
    #[arg(long)]
    folder: String,
}

#[derive(Args)]
struct NotesSearchArgs {
    /// Target account name (default: first account)
    #[arg(long)]
    account: Option<String>,
    /// Query string (name/body contains)
    #[arg(long)]
    query: String,
    /// Optional limit (0 = no limit)
    #[arg(long, default_value = "0")]
    limit: usize,
}

#[derive(Args)]
struct NotesShowArgs {
    /// Note id
    id: String,
}

#[derive(Args)]
struct NotesAttachmentsCmd {
    #[command(subcommand)]
    command: NotesAttachmentsSubcommand,
}

#[derive(Subcommand)]
enum NotesAttachmentsSubcommand {
    List(NotesAttachmentsListArgs),
    Save(NotesAttachmentsSaveArgs),
    Delete(NotesAttachmentsDeleteArgs),
}

#[derive(Args)]
struct NotesAttachmentsListArgs {
    /// Note id
    id: String,
}

#[derive(Args)]
struct NotesAttachmentsSaveArgs {
    /// Note id
    id: String,
    /// Attachment id
    #[arg(long)]
    attachment_id: Option<String>,
    /// Attachment name
    #[arg(long)]
    name: Option<String>,
    /// Output directory (POSIX path)
    #[arg(long)]
    output: String,
}

#[derive(Args)]
struct NotesAttachmentsDeleteArgs {
    /// Note id
    id: String,
    /// Attachment id
    #[arg(long)]
    attachment_id: Option<String>,
    /// Attachment name
    #[arg(long)]
    name: Option<String>,
}

#[derive(Args)]
struct RemindersListArgs {
    /// List name (default: first list)
    #[arg(long)]
    list: Option<String>,
    /// Optional limit (0 = no limit)
    #[arg(long, default_value = "0")]
    limit: usize,
    /// Only completed reminders (true/false). If omitted, all.
    #[arg(long)]
    completed: Option<bool>,
}

#[derive(Args)]
struct RemindersListCreateArgs {
    /// Account name (default: first account)
    #[arg(long)]
    account: Option<String>,
    /// List name
    #[arg(long)]
    name: String,
    /// List color
    #[arg(long)]
    color: Option<String>,
    /// Emblem/icon name
    #[arg(long)]
    emblem: Option<String>,
}

#[derive(Args)]
struct RemindersListUpdateArgs {
    /// List name (existing)
    #[arg(long)]
    name: String,
    /// New list name
    #[arg(long)]
    new_name: Option<String>,
    /// List color
    #[arg(long)]
    color: Option<String>,
    /// Emblem/icon name
    #[arg(long)]
    emblem: Option<String>,
}

#[derive(Args)]
struct RemindersListDeleteArgs {
    /// List name
    #[arg(long)]
    name: String,
}

#[derive(Args)]
struct RemindersGetArgs {
    /// Reminder id
    id: String,
}

#[derive(Args)]
struct RemindersCreateArgs {
    /// List name (default: first list)
    #[arg(long)]
    list: Option<String>,
    /// Parent reminder id (create subtask)
    #[arg(long)]
    parent: Option<String>,
    /// Reminder title
    #[arg(long)]
    title: String,
    /// Reminder notes/body
    #[arg(long)]
    body: Option<String>,
    /// Due date (string parsed by AppleScript date)
    #[arg(long)]
    due: Option<String>,
    /// All-day due date (date only)
    #[arg(long)]
    allday_due: Option<String>,
    /// Remind me date/time
    #[arg(long)]
    remind_me: Option<String>,
    /// Priority (0-9)
    #[arg(long)]
    priority: Option<u8>,
    /// Flagged (true/false)
    #[arg(long)]
    flagged: Option<bool>,
}

#[derive(Args)]
struct RemindersUpdateArgs {
    /// Reminder id
    id: String,
    /// New title
    #[arg(long)]
    title: Option<String>,
    /// New body
    #[arg(long)]
    body: Option<String>,
    /// New due date (string parsed by AppleScript date)
    #[arg(long)]
    due: Option<String>,
    /// New all-day due date
    #[arg(long)]
    allday_due: Option<String>,
    /// New remind me date/time
    #[arg(long)]
    remind_me: Option<String>,
    /// New priority (0-9)
    #[arg(long)]
    priority: Option<u8>,
    /// Mark completed (true/false)
    #[arg(long)]
    completed: Option<bool>,
    /// Flagged (true/false)
    #[arg(long)]
    flagged: Option<bool>,
}

#[derive(Args)]
struct RemindersCompleteArgs {
    /// Reminder id
    id: String,
}

#[derive(Args)]
struct RemindersDeleteArgs {
    /// Reminder id
    id: String,
}

#[derive(Args)]
struct CalendarEventsArgs {
    /// Calendar name (default: first calendar)
    #[arg(long)]
    calendar: Option<String>,
    /// Start date/time (AppleScript-parsed, default: today 00:00)
    #[arg(long)]
    start: Option<String>,
    /// End date/time (AppleScript-parsed, default: start + 1 day)
    #[arg(long)]
    end: Option<String>,
    /// Optional limit (0 = no limit)
    #[arg(long, default_value = "0")]
    limit: usize,
    /// Optional query (summary contains)
    #[arg(long)]
    query: Option<String>,
}

#[derive(Args)]
struct CalendarCalendarsCreateArgs {
    /// Calendar name
    #[arg(long)]
    name: String,
    /// Calendar description
    #[arg(long)]
    description: Option<String>,
}

#[derive(Args)]
struct CalendarCalendarsDeleteArgs {
    /// Calendar name
    #[arg(long)]
    name: String,
}

#[derive(Args)]
struct CalendarGetArgs {
    /// Event UID
    id: String,
}

#[derive(Args)]
struct CalendarCreateArgs {
    /// Calendar name (default: first calendar)
    #[arg(long)]
    calendar: Option<String>,
    /// Event title/summary
    #[arg(long)]
    title: String,
    /// Start date/time (AppleScript-parsed)
    #[arg(long)]
    start: String,
    /// End date/time (AppleScript-parsed)
    #[arg(long)]
    end: Option<String>,
    /// All-day event
    #[arg(long)]
    allday: bool,
    /// Location
    #[arg(long)]
    location: Option<String>,
    /// Notes/description
    #[arg(long)]
    notes: Option<String>,
    /// URL
    #[arg(long)]
    url: Option<String>,
    /// Recurrence rule (RFC 2445, e.g. "RRULE:FREQ=WEEKLY;INTERVAL=1")
    #[arg(long)]
    recurrence: Option<String>,
    /// Status: confirmed|tentative|cancelled
    #[arg(long)]
    status: Option<String>,
}

#[derive(Args)]
struct CalendarUpdateArgs {
    /// Event UID
    id: String,
    /// New title/summary
    #[arg(long)]
    title: Option<String>,
    /// New start date/time
    #[arg(long)]
    start: Option<String>,
    /// New end date/time
    #[arg(long)]
    end: Option<String>,
    /// All-day event
    #[arg(long)]
    allday: Option<bool>,
    /// Location
    #[arg(long)]
    location: Option<String>,
    /// Notes/description
    #[arg(long)]
    notes: Option<String>,
    /// URL
    #[arg(long)]
    url: Option<String>,
    /// Recurrence rule (RFC 2445)
    #[arg(long)]
    recurrence: Option<String>,
    /// Status: confirmed|tentative|cancelled
    #[arg(long)]
    status: Option<String>,
}

#[derive(Args)]
struct CalendarDeleteArgs {
    /// Event UID
    id: String,
}

#[derive(Args)]
struct CalendarShowArgs {
    /// Event UID
    id: String,
}

#[derive(Args)]
struct CalendarAlarmsCmd {
    #[command(subcommand)]
    command: CalendarAlarmsSubcommand,
}

#[derive(Subcommand)]
enum CalendarAlarmsSubcommand {
    List(CalendarAlarmsListArgs),
    Add(CalendarAlarmsAddArgs),
    Delete(CalendarAlarmsDeleteArgs),
}

#[derive(Args)]
struct CalendarAlarmsListArgs {
    /// Event UID
    id: String,
}

#[derive(Args)]
struct CalendarAlarmsAddArgs {
    /// Event UID
    id: String,
    /// Alarm type: display|mail|sound
    #[arg(long)]
    r#type: String,
    /// Trigger interval in minutes (negative = before event)
    #[arg(long)]
    minutes: Option<i32>,
    /// Absolute trigger date (YYYY-MM-DD or YYYY-MM-DD HH:MM:SS)
    #[arg(long)]
    date: Option<String>,
    /// Sound name (sound alarm only)
    #[arg(long)]
    sound_name: Option<String>,
    /// Sound file path (sound alarm only)
    #[arg(long)]
    sound_file: Option<String>,
}

#[derive(Args)]
struct CalendarAlarmsDeleteArgs {
    /// Event UID
    id: String,
    /// Alarm type: display|mail|sound
    #[arg(long)]
    r#type: String,
    /// 1-based index within the selected alarm type
    #[arg(long)]
    index: usize,
}

#[derive(Args)]
struct CalendarAttendeesCmd {
    #[command(subcommand)]
    command: CalendarAttendeesSubcommand,
}

#[derive(Subcommand)]
enum CalendarAttendeesSubcommand {
    List(CalendarAttendeesListArgs),
    Add(CalendarAttendeesAddArgs),
}

#[derive(Args)]
struct CalendarAttendeesListArgs {
    /// Event UID
    id: String,
}

#[derive(Args)]
struct CalendarAttendeesAddArgs {
    /// Event UID
    id: String,
    /// Attendee email
    #[arg(long)]
    email: String,
}

#[derive(Args)]
struct MessagesBuddiesArgs {
    /// Service name (default: first service)
    #[arg(long)]
    service: Option<String>,
    /// Service type: imessage or sms
    #[arg(long)]
    r#type: Option<String>,
}

#[derive(Args)]
struct MessagesSendArgs {
    /// Recipient handle (phone number or email)
    #[arg(long)]
    to: String,
    /// Message text
    #[arg(long)]
    text: Option<String>,
    /// File path to send
    #[arg(long)]
    file: Option<String>,
    /// Service name (default: first service)
    #[arg(long)]
    service: Option<String>,
    /// Service type: imessage or sms
    #[arg(long)]
    r#type: Option<String>,
}

#[derive(Args)]
struct MessagesChatsArgs {
    /// Service name (default: all)
    #[arg(long)]
    service: Option<String>,
    /// Service type: imessage or sms or rcs
    #[arg(long)]
    r#type: Option<String>,
}

#[derive(Args)]
struct MessagesChatParticipantsArgs {
    /// Chat id
    #[arg(long)]
    id: Option<String>,
    /// Chat name
    #[arg(long)]
    name: Option<String>,
}

#[derive(Args)]
struct MessagesSendChatArgs {
    /// Chat id
    #[arg(long)]
    id: Option<String>,
    /// Chat name
    #[arg(long)]
    name: Option<String>,
    /// Message text
    #[arg(long)]
    text: Option<String>,
    /// File path to send
    #[arg(long)]
    file: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        TopCommand::Notes(cmd) => match cmd.command {
            NotesSubcommand::Accounts(cmd) => match cmd.command {
                AccountsSubcommand::List => notes::accounts_list(),
            },
            NotesSubcommand::Folders(cmd) => match cmd.command {
                FoldersSubcommand::List(args) => notes::folders_list(args),
                FoldersSubcommand::Create(args) => notes::folders_create(args),
                FoldersSubcommand::Delete(args) => notes::folders_delete(args),
            },
            NotesSubcommand::List(args) => notes::notes_list(args),
            NotesSubcommand::Get(args) => notes::notes_get(args),
            NotesSubcommand::Create(args) => notes::notes_create(args),
            NotesSubcommand::Update(args) => notes::notes_update(args),
            NotesSubcommand::Delete(args) => notes::notes_delete(args),
            NotesSubcommand::Move(args) => notes::notes_move(args),
            NotesSubcommand::Search(args) => notes::notes_search(args),
            NotesSubcommand::Show(args) => notes::notes_show(args),
            NotesSubcommand::Attachments(cmd) => match cmd.command {
                NotesAttachmentsSubcommand::List(args) => notes::notes_attachments_list(args),
                NotesAttachmentsSubcommand::Save(args) => notes::notes_attachments_save(args),
                NotesAttachmentsSubcommand::Delete(args) => notes::notes_attachments_delete(args),
            },
        },
        TopCommand::Reminders(cmd) => match cmd.command {
            RemindersSubcommand::Lists => reminders::reminders_lists(),
            RemindersSubcommand::ListsCreate(args) => reminders::reminders_lists_create(args),
            RemindersSubcommand::ListsUpdate(args) => reminders::reminders_lists_update(args),
            RemindersSubcommand::ListsDelete(args) => reminders::reminders_lists_delete(args),
            RemindersSubcommand::List(args) => reminders::reminders_list(args),
            RemindersSubcommand::Get(args) => reminders::reminders_get(args),
            RemindersSubcommand::Create(args) => reminders::reminders_create(args),
            RemindersSubcommand::Update(args) => reminders::reminders_update(args),
            RemindersSubcommand::Complete(args) => reminders::reminders_complete(args),
            RemindersSubcommand::Delete(args) => reminders::reminders_delete(args),
        },
        TopCommand::Calendar(cmd) => match cmd.command {
            CalendarSubcommand::Calendars => calendar::calendar_calendars(),
            CalendarSubcommand::CalendarsCreate(args) => calendar::calendar_calendars_create(args),
            CalendarSubcommand::CalendarsDelete(args) => calendar::calendar_calendars_delete(args),
            CalendarSubcommand::Events(args) => calendar::calendar_events(args),
            CalendarSubcommand::Get(args) => calendar::calendar_get(args),
            CalendarSubcommand::Create(args) => calendar::calendar_create(args),
            CalendarSubcommand::Update(args) => calendar::calendar_update(args),
            CalendarSubcommand::Delete(args) => calendar::calendar_delete(args),
            CalendarSubcommand::Show(args) => calendar::calendar_show(args),
            CalendarSubcommand::Alarms(cmd) => match cmd.command {
                CalendarAlarmsSubcommand::List(args) => calendar::calendar_alarms_list(args),
                CalendarAlarmsSubcommand::Add(args) => calendar::calendar_alarms_add(args),
                CalendarAlarmsSubcommand::Delete(args) => calendar::calendar_alarms_delete(args),
            },
            CalendarSubcommand::Attendees(cmd) => match cmd.command {
                CalendarAttendeesSubcommand::List(args) => calendar::calendar_attendees_list(args),
                CalendarAttendeesSubcommand::Add(args) => calendar::calendar_attendees_add(args),
            },
        },
        TopCommand::Messages(cmd) => match cmd.command {
            MessagesSubcommand::Services => messages::messages_services(),
            MessagesSubcommand::Buddies(args) => messages::messages_buddies(args),
            MessagesSubcommand::Send(args) => messages::messages_send(args),
            MessagesSubcommand::Chats(args) => messages::messages_chats(args),
            MessagesSubcommand::ChatParticipants(args) => messages::messages_chat_participants(args),
            MessagesSubcommand::SendChat(args) => messages::messages_send_chat(args),
        },
    }
}
