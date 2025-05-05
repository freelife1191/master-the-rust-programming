# Exercise-diy-12

본 강의에는 재고관리 어플리케이션의 일부 코드가 첨부되어 있다  
코드는 현재 오류 처리를 위해 `panic!()`을 사용한다. `panic!`의 모든 인스턴스를 대체하려면 코드를 리팩터링해야 한다    
**Result** 열거형과 사용자 정의 오류 유형 **InventoryError**를 사용하여 적절한 오류 처리를 수행한다  
**Inventory** 구조체의 각 메서드가 **Result** 유형을 반환하여 의미 있는 오류 메시지를 제공하고 오류를 적절하게 처리하는지 확인하라

1) Create a new project : `cargo new inventory`

2) copy the contents of `inventory.rs` file to `main.rs` file created in the `inventory/src` directory
