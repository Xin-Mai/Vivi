package com.example.vivi.controller;

import com.example.vivi.pojo.Article;
import com.example.vivi.service.ArticleService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;

@RestController
public class ArticleController {
    @Autowired
    ArticleService articleService;


    @GetMapping("/api/test")
    public List<Article> mainPageList() throws Exception{
        System.out.println("in mainPage");
        return articleService.list();
    }

}
