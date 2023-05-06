use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::path::Path;

pub async fn async_watch<P: AsRef<Path>>(
    path: P,
) -> Result<(), Box<dyn Send + Sync + std::error::Error>> {
    let (mut tx, mut rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Default::default(),
    )?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => match event.kind {
                EventKind::Create(CreateKind::File)
                | EventKind::Modify(ModifyKind::Name(RenameMode::To))
                | EventKind::Remove(RemoveKind::File) => {
                    if event
                        .paths
                        .iter()
                        .any(|v| v.extension().unwrap_or_default().eq("acf"))
                    {
                        println!("FOUND EVENT WE CARE ABOUT: {:?}", event);
                    }
                }
                _ => {},
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
