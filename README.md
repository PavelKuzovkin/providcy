### Реализованная функциональность
* блокчейн для хранения данных кредитных договоров и договоров страхования;
* АРМ сотрудника банка;
* АРМ сотрудника страховой компании;

### Особенность проекта в следующем:
* Система нотификации заемщиков об окончании срока договора страхования;

### Основной стек технологий:
* HTML, CSS, JavaScript, TypeScript.

### Демо
Демо сервиса доступно по адресу: http://185.174.235.90

Реквизиты тестового пользователя: login: user1, пароль: password

### СРЕДА ЗАПУСКА
развертывание сервиса производится в docker-контейнерах;

требуется установленные пакеты docker и docker-compose

### УСТАНОВКА
В среде Ubuntu выполните:

```
sudo apt update
sudo apt upgrade
sudo apt install docker
sudo apt install docker-compose
git clone https://github.com/PavelKuzovkin/providcy.git
cd ./providcy
sudo sh install
```

### РАЗРАБОТЧИКИ
Александр Мариненко Blockchain

Роман Стадников Frontend

Максим Провоторов python, RabbitMQ

