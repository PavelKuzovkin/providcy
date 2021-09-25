package ru.providcy.dto.table;

import lombok.AllArgsConstructor;
import lombok.Data;

@Data
@AllArgsConstructor
public class TokenStateDto {
    private String idToken;
    private String state;
}
