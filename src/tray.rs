use ksni;

#[derive(Debug)]
pub struct DragonCenterTray {}

impl ksni::Tray for DragonCenterTray {
    fn title(&self) -> String {
        "hello, world".into()
    }
}

pub fn start_tray() -> Result<(), Box<dyn std::error::Error>> {
    let service = ksni::TrayService::new(DragonCenterTray {});
    let _ = service.handle();
    service.spawn();

    Ok(())
}
