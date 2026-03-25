use crate::Language;

pub struct Localization {}

impl Localization {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get<'a>(&self, lang: &Language, key: &'a str) -> &'a str {
        match lang {
            Language::English => match key {
                "title" => "Rust Optimizer",
                "menu_title" => " Optimizer Menu (Enter to run) [L] to switch lang ",
                "logs_title" => " Action Logs ",
                "item_clean_pc" => "Clean PC (Temp / Prefetch)",
                "item_kill_bloat" => "Kill Background Processes & Telemetry",
                "item_opt_rust" => "Optimize Rust (client.cfg)",
                "item_prio_rust" => "Set RustClient.exe to High Priority",
                "item_clean_ram" => "Clean RAM (Standby List Purge)",
                "item_lossless_scaling" => "Apply Lossless Scaling (Borderless Fullscreen)",
                "item_network_opt" => "Optimize Network (Throttling & TcpAckFrequency)",
                "item_power_plan" => "Enable Ultimate Performance Power Plan",
                "item_disable_gamedvr" => "Disable Xbox Game DVR & Overlays",
                "item_timer_resolution" => "Force 1ms Timer Resolution (Lower Input Lag)",
                "item_quit" => "Quit",
                "welcome" => "Welcome to Rust Optimizer!",
                "deleted" => "Deleted",
                "files" => "files",
                "bytes" => "bytes.",
                "killed_processes" => "Killed bloatware/telemetry processes.",
                _ => key,
            },
            Language::Ukrainian => match key {
                "title" => "Оптимізатор Rust",
                "menu_title" => " Меню (Enter для запуску) [L] змінити мову ",
                "logs_title" => " Логи Дій ",
                "item_clean_pc" => "Очистити ПК (Temp / Prefetch)",
                "item_kill_bloat" => "Вбити фонові процеси і телеметрію",
                "item_opt_rust" => "Оптимізувати Rust (client.cfg)",
                "item_prio_rust" => "Надати RustClient.exe високий пріоритет",
                "item_clean_ram" => "Очистити ОЗП (Standby List Purge)",
                "item_lossless_scaling" => "Застосувати Lossless Scaling (Безрамковий екран)",
                "item_network_opt" => "Оптимізувати мережу (Зниження пінгів)",
                "item_power_plan" => "Увімкнути план Максимальна продуктивність",
                "item_disable_gamedvr" => "Вимкнути Xbox Game DVR та оверлеї",
                "item_timer_resolution" => "Встановити таймер на 1ms (Менший Input Lag)",
                "item_quit" => "Вихід",
                "welcome" => "Ласкаво просимо в Оптимізатор Rust!",
                "deleted" => "Видалено ",
                "files" => "файлів",
                "bytes" => "байт.",
                "killed_processes" => "Вбито фонових процесів/телеметрії.",
                _ => key,
            },
            Language::Russian => match key {
                "title" => "Оптимизатор Rust",
                "menu_title" => " Меню Оптимизатора (Enter для запуска) [L] яз ",
                "logs_title" => " Логи Действий ",
                "item_clean_pc" => "Очистить ПК (Temp / Prefetch)",
                "item_kill_bloat" => "Убить фоновые процессы и телеметрию",
                "item_opt_rust" => "Оптимизировать Rust (client.cfg)",
                "item_prio_rust" => "Выдать RustClient.exe высокий приоритет",
                "item_clean_ram" => "Очистить ОЗУ (Standby List Purge)",
                "item_lossless_scaling" => "Применить Lossless Scaling (Безрамочный экран)",
                "item_network_opt" => "Оптимизировать сеть (Снижение пинга)",
                "item_power_plan" => "Включить план Максимальная производительность",
                "item_disable_gamedvr" => "Отключить Xbox Game DVR и оверлеи",
                "item_timer_resolution" => "Установить таймер на 1ms (Меньший Input Lag)",
                "item_quit" => "Выход",
                "welcome" => "Добро пожаловать в Rust Optimizer!",
                "deleted" => "Удалено",
                "files" => "файлов",
                "bytes" => "байт.",
                "killed_processes" => "Убиты фоновые процессы и службы телеметрии.",
                _ => key,
            },
        }
    }
}
