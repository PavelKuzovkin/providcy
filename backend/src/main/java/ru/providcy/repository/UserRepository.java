package ru.providcy.repository;

import org.springframework.data.jpa.repository.JpaRepository;
import ru.providcy.model.User;

import java.util.Optional;

public interface UserRepository extends JpaRepository<User, Long> {

    Optional<User> findByLoginAndPasswd(String login, String pass);

}
