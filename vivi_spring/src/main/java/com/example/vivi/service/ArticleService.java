package com.example.vivi.service;

import com.example.vivi.dao.ArticleDAO;
import com.example.vivi.dao.UserDAO;
import com.example.vivi.pojo.Article;
import com.example.vivi.pojo.User;
import org.springframework.beans.factory.annotation.Autowired;

import org.springframework.data.domain.Sort;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class ArticleService {
    @Autowired
    ArticleDAO articleDAO;
    @Autowired
    UserDAO userDAO;

    //获取首页的文章
    public List<Article> list() {
        Sort sort = Sort.by(Sort.Direction.DESC,"id");
        return articleDAO.findAll();
    }

    //添加文章或者修改
    public void addOrUpdate(Article article){
        articleDAO.save(article);
    }
    //删除文章
    public void deleteById(int id){
        articleDAO.deleteById(id);
    }

    //显示一个作者的全部文章
    public List<Article> getAllArticlesByAuthor(int id){
        User author = userDAO.findById(id);
        return articleDAO.findAllByAuthor(author);
    }

    //根据标题搜索文章
    public List<Article> getArticlesByTitle(String keyword){
        return articleDAO.findAllByTitleLike(keyword);
    }
}
