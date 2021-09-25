package ru.providcy.repository;

import org.springframework.data.jpa.repository.JpaRepository;
import ru.providcy.model.Session;

import java.util.Optional;

public interface SessionRepository extends JpaRepository<Session, Long> {

    Optional<Session> findByUserId(Long userId);

    Optional<Session> findByToken(String token);
}
