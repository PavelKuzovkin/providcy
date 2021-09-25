package ru.providcy.service;

import lombok.AllArgsConstructor;
import org.springframework.stereotype.Service;
import ru.providcy.dto.in.LoginDto;
import ru.providcy.dto.table.TokenDto;
import ru.providcy.model.Session;
import ru.providcy.model.User;
import ru.providcy.repository.SessionRepository;
import ru.providcy.repository.UserRepository;

import java.time.Instant;
import java.util.Optional;
import java.util.UUID;

@Service
@AllArgsConstructor
public class AuthService {

    private final UserRepository userRepository;
    private final SessionRepository sessionRepository;

    public TokenDto login(LoginDto dto) {
        Optional<User> optionalUser = userRepository.findByLoginAndPasswd(dto.getLogin(), dto.getPassword());
        if (optionalUser.isEmpty()) return null;

        User user = optionalUser.get();

        Optional<Session> optionalSession = sessionRepository.findByUserId(user.getId());
        Session session = null;

        if (optionalSession.isPresent()) {
            session = optionalSession.get();

            if (session.getCreatedAt().plusSeconds(session.getExpiresIn()).isBefore(Instant.now())) {
                sessionRepository.delete(session);
                session = null;
            }
        }

        if (session == null) {
            session = new Session(
                    null,
                    user,
                    UUID.randomUUID().toString(),
                    Instant.now(),
                    3600L
            );

            sessionRepository.save(session);
        }

        TokenDto tokenDto = new TokenDto(
                user.getId(),
                user.getFirstName() + " " + user.getPatronymic() + " " + user.getSurname(),
                session.getToken(),
                session.getExpiresIn()
        );

        return tokenDto;
    }

    public void logout(String token) {
        Optional<Session> optional = sessionRepository.findByToken(token);
        optional.ifPresent(sessionRepository::delete);
    }

    public Long checkSession(String token) {
        Optional<Session> optional = sessionRepository.findByToken(token);

        if (optional.isEmpty()) return null;

        Session session = optional.get();

        if (session.getCreatedAt().plusSeconds(session.getExpiresIn()).isBefore(Instant.now())) {
            sessionRepository.delete(session);
            return null;
        } else {
            return session.getUser().getId();
        }
    }

}
