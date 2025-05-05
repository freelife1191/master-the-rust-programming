# Privacy rules (프라이버시 규칙)

1.  **Default Privacy (기본 프라이버시):**
    * 모든 `item` (`functions`, `methods`, `structs`, `enums`, `modules`, `constants`)은 기본적으로 `private` 임
2.  **Parent to Child Module Access (부모 -> 자식 Module 접근):**
    * 부모 `module` 의 `item` 은 자식 `module` 내부의 `private` `item` 에 접근할 수 없음
3.  **Child to Parent (Ancestor) Module Access (자식 -> 부모(조상) Module 접근):**
    * 자식 `module` 의 `item` 은 조상 `module` (직계 부모뿐만 아니라 계층 구조 상위의 모든 `module`) 내의 `item` 들을 사용할 수 있음
4.  **Sibling Module Access (형제 Module 접근):**
    * 형제 `module` 은 기본적으로 서로의 `private` `item` 에 접근할 수 없음