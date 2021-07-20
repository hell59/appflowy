use derive_more::Display;
use flowy_derive::{Flowy_Event, ProtoBuf_Enum};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Display, Hash, ProtoBuf_Enum, Flowy_Event)]
#[event_err = "WorkspaceError"]
pub enum WorkspaceEvent {
    #[display(fmt = "Create workspace")]
    #[event(input = "CreateSpaceRequest", output = "WorkspaceDetail")]
    CreateWorkspace    = 0,

    #[display(fmt = "Get user's workspace detail")]
    #[event(output = "UserWorkspaceDetail")]
    GetWorkspaceDetail = 1,

    #[display(fmt = "Create app")]
    #[event(input = "CreateAppRequest", output = "App")]
    CreateApp          = 101,

    #[display(fmt = "Create view")]
    #[event(input = "CreateViewRequest", output = "View")]
    CreateView         = 201,
}