// tests/hardware_tests.rs
#[cfg(test)]
mod hardware_tests {
    use usmc::*;
    use std::thread;
    use std::time::Duration;

    /// Тестирование обнаружения оборудования
    #[test]
    fn test_hardware_detection() {
        println!("Тестирование обнаружения оборудования...");
        
        // 1. Создаем контроллер
        let mut controller = match USMCController::new() {
            Ok(c) => {
                println!("Контроллер создан");
                c
            }
            Err(e) => {
                println!("Не удалось создать контроллер: {}", e);
                return;
            }
        };

        // 2. Инициализация и поиск устройств
        println!("Инициализация оборудования...");
        match controller.initialize() {
            Ok(devices) => {
                println!("Инициализация успешна");
                println!("Найдено устройств: {}", devices.len());
                
                if devices.is_empty() {
                    println!("Устройства не обнаружены!");
                    return;
                }

                for (i, (serial, version)) in devices.iter().enumerate() {
                    println!("Устройство #{}:", i);
                    println!("Серийный номер: {}", serial);
                    println!("Версия: {}", version);
                    
                    match controller.get_device_state(i as u32) {
                        Ok(state) => {
                            println!("     Состояние:");
                            println!("       Позиция: {}", state.CurPos);
                            println!("       Температура: {:.1}°C", state.Temp);
                            println!("       Напряжение: {:.1}В", state.Voltage);
                            println!("       Работает: {}", state.RUN);
                            println!("       Питание: {}", state.Power);
                            println!("       Скорость: {}", if state.FullSpeed { "полная" } else { "ограниченная" });
                        }
                        Err(e) => {
                            println!("Не удалось получить состояние: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Ошибка инициализации: {}", e);
            }
        }
    }

    #[test]
    fn test_device_detailed_info() {
        println!("Тестирование  информации об устройстве...");
        
        let controller = match USMCController::new() {
            Ok(c) => c,
            Err(_) => return,
        };

        let device_id = 0;
        
        // Получаем состояние
        match controller.get_device_state(device_id) {
            Ok(state) => {
                println!("Состояние устройства #{}:", device_id);
                println!("   Позиция: {}", state.CurPos);
                println!("   Температура контроллера: {:.1}°C", state.Temp);
                println!("   Напряжение питания: {:.2}В", state.Voltage);
                println!("   Состояние питания: {}", if state.Power { "ВКЛ" } else { "ВЫКЛ" });
                println!("   Двигатель: {}", if state.RUN { "работает" } else { "остановлен" });
                println!("   Направление: {}", if state.CW_CCW { "CW" } else { "CCW" });
                println!("   Температурный режим: {}", if state.FullPower { "полная мощность" } else { "ограниченная" });
                println!("     Trailer1: {}", state.Trailer1);
                println!("     Trailer2: {}", state.Trailer2);
                println!("   Сигналы синхронизации:");
                println!("     SyncIN: {}", state.SyncIN);
                println!("     SyncOUT: {}", state.SyncOUT);
            }
            Err(e) => {
                println!("Не удалось получить состояние: {}", e);
            }
        }
    }

    #[test]
    fn test_movement() {
        println!("Тестирование плавного движения...");
        
        let controller = match USMCController::new() {
            Ok(c) => c,
            Err(_) => return,
        };

        let device_id = 0;
        
        let initial_state = match controller.get_device_state(device_id) {
            Ok(state) => {
                if state.RUN {
                    println!("Двигатель уже работает. Останавливаем...");
                    let _ = controller.stop_motor(device_id);
                    thread::sleep(Duration::from_millis(100));
                }
                
                if state.Trailer1 || state.Trailer2 {
                    println!("Пропускаем тест движения.");
                    return;
                }
                
                state
            }
            Err(_) => return,
        };

        let initial_pos = initial_state.CurPos;
        println!("Начальная позиция: {}", initial_pos);

        println!("Движение вперед на 30 ");
        let target_pos_forward = initial_pos + 30;
        let speed = 3.0; 

        match controller.start_motor(device_id, target_pos_forward, speed) {
            Ok(actual_speed) => {
                println!("   Запущено со скоростью: {:.1} шаг/сек", actual_speed);
                
                thread::sleep(Duration::from_millis(2000));
                
                if let Ok(state) = controller.get_device_state(device_id) {
                    println!("Текущая позиция: {} шагов", state.CurPos);
                    if !state.RUN {
                        println!("Движение завершено");
                    } else {
                        println!("Движение еще не завершено");
                        let _ = controller.stop_motor(device_id);
                    }
                }
            }
            Err(e) => {
                println!("Ошибка запуска: {}", e);
            }
        }

        thread::sleep(Duration::from_millis(500));

        println!("Движение назад на 20 ");
        let current_state = controller.get_device_state(device_id).unwrap();
        let target_pos_backward = current_state.CurPos - 20;

        match controller.start_motor(device_id, target_pos_backward, speed) {
            Ok(actual_speed) => {
                println!("Запущено со скоростью: {:.1} шаг/сек", actual_speed);
                thread::sleep(Duration::from_millis(1500));
                let _ = controller.stop_motor(device_id);
            }
            Err(e) => {
                println!("Ошибка запуска: {}", e);
            }
        }

        println!("Возврат в исходную позицию...");
        thread::sleep(Duration::from_millis(500));
        let _ = controller.start_motor(device_id, initial_pos, speed);
        thread::sleep(Duration::from_millis(2000));
        let _ = controller.stop_motor(device_id);

        println!("Тест движения завершен");
    }

    #[test]
    fn test_temperature_monitoring() {
        println!("Тестирование температуры");
        
        let controller = match USMCController::new() {
            Ok(c) => c,
            Err(_) => return,
        };

        let device_id = 0;
        
        println!("Мониторинг температуры в течение 10 секунд...");
        
        for i in 1..=10 {
            match controller.get_device_state(device_id) {
                Ok(state) => {
                    println!("   {} сек: {:.1}°C", i, state.Temp);
                }

                Err(e) => {
                    println!("Ошибка чтения температуры: {}", e);
                    break;
                }
            }
            
            if i < 10 {
                thread::sleep(Duration::from_secs(1));
            }
        }
        
        println!("Мониторинг температуры завершен");
    }
}