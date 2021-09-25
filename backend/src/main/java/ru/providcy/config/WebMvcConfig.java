package ru.providcy.config;

import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.EnableWebMvc;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurerAdapter;

@Configuration
@EnableWebMvc
public class WebMvcConfig extends WebMvcConfigurerAdapter {

    @Override
    public void addCorsMappings(CorsRegistry registry) {
        registry.addMapping("/**").allowedOrigins("*")
        .allowedMethods("GET", "POST", "OPTIONS", "PUT", "DELETE")
        .allowedHeaders("Content-Type", "X-Requested-With", "accept", "Origin", "Access-Control-Request-Methods", "Access-Control-Request-Headers", "Access-Control-Allow-Origin");
//                .exposedHeaders("Access-Control-Allow-Credentials")
//                .allowCredentials(true).maxAge(3600);
    }
}
