package com.example.vivi.wrapper;

import com.example.vivi.pojo.Article;

import java.text.SimpleDateFormat;
import java.util.Date;

public class ArticleWrapper {
    private static SimpleDateFormat format = new SimpleDateFormat("yyyy-mm-dd hh:mm");
    private int id;
    private String title;
    private String content;
    private String publishDate;
    private UserWrapper author;
    private int likeNum;
    private int readNum;
    public ArticleWrapper(Article article){
        this.id = article.getId();
        this.title = article.getTitle();
        this.content = article.getContent();
        this.likeNum = article.getLikeNum();
        this.readNum = article.getReadNum();
        this.publishDate = format.format(article.getPublishDate());
        this.author = new UserWrapper(article.getAuthor());
    }

    public int getId() {
        return id;
    }

    public void setId(int id) {
        this.id = id;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public String getContent() {
        return content;
    }

    public void setContent(String content) {
        this.content = content;
    }

    public String getPublishDate() {
        return publishDate;
    }

    public void setPublishDate(String publishDate) {
        this.publishDate = publishDate;
    }

    public UserWrapper getAuthor() {
        return author;
    }

    public void setAuthor(UserWrapper author) {
        this.author = author;
    }

    public int getLikeNum() {
        return likeNum;
    }

    public void setLikeNum(int likeNum) {
        this.likeNum = likeNum;
    }

    public int getReadNum() {
        return readNum;
    }

    public void setReadNum(int readNum) {
        this.readNum = readNum;
    }
}
