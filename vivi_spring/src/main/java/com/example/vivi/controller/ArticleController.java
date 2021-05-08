package com.example.vivi.controller;

import com.example.vivi.pojo.Article;
import com.example.vivi.service.ArticleService;
import com.example.vivi.wrapper.ArticleWrapper;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.List;
import java.util.logging.Logger;

@RestController
public class ArticleController {
    @Autowired
    ArticleService articleService;
    private static Logger log = Logger.getLogger(ArticleController.class.getName());


    @GetMapping("/api/test")
    public List<Article> mainPageList() throws Exception{
        System.out.println("in mainPage");
        return articleService.list();
    }

    @CrossOrigin
    @ResponseBody
    @GetMapping("/api/article/{id}")
    public ArticleWrapper getArticleById(@PathVariable("id") int id) throws Exception{
        Article article = articleService.getById(id);
        if (article!=null)
            return new ArticleWrapper(article);
        return null;

    }

    @CrossOrigin
    @ResponseBody
    @PostMapping("/api/publish")
    public int publish(@RequestBody Article article) throws Exception{
        log.info("article: "+article);
        /*SimpleDateFormat sdf = new SimpleDateFormat();// 格式化时间
        sdf.applyPattern("yyyy-MM-dd HH:mm:ss");*/
        article.setPublishDate(new Date());
        articleService.addOrUpdate(article);
        return article.getId();
    }

}
