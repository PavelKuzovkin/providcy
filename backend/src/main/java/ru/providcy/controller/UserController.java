package ru.providcy.controller;

import lombok.AllArgsConstructor;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import ru.providcy.dto.in.LoginDto;
import ru.providcy.dto.table.TokenDto;
import ru.providcy.service.AuthService;

@RestController
@AllArgsConstructor
@RequestMapping("/api/v1/user")
public class UserController {

    private final AuthService authService;

    @PostMapping("/login")
    public ResponseEntity<TokenDto> login(@RequestBody LoginDto dto) {
        TokenDto tokenDto = authService.login(dto);

        if (tokenDto == null)
            return new ResponseEntity(HttpStatus.UNAUTHORIZED);
        else
            return ResponseEntity.ok(tokenDto);
    }

    @PostMapping("/logout")
    public ResponseEntity logout(@RequestParam(value = "auth") String idToken) {
        authService.logout(idToken);
        return new ResponseEntity(HttpStatus.UNAUTHORIZED);
    }

    @GetMapping("/token_state")
    public ResponseEntity tokenState(@RequestParam(value = "auth") String idToken) {
        Long userId = authService.checkSession(idToken);
        if (userId == null) return new ResponseEntity(HttpStatus.UNAUTHORIZED);

        return new ResponseEntity(HttpStatus.OK);
    }
}
