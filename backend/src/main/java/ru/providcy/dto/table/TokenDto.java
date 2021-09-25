package ru.providcy.dto.table;

import lombok.AllArgsConstructor;
import lombok.Data;
import ru.providcy.model.User;

@Data
@AllArgsConstructor
public class TokenDto {
    private long userId;
    private String displayName;
    private String idToken;
    private long expiresIn;

    public TokenDto(User dto) {
        this.userId = dto.getId();
        this.displayName = dto.getFirstName() + " " + dto.getPatronymic() + " " + dto.getSurname();
        idToken = null;
        expiresIn = 0;
    }
}
