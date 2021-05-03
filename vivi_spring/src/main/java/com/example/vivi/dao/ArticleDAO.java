package com.example.vivi.dao;

import com.example.vivi.pojo.Article;
import com.example.vivi.pojo.User;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface ArticleDAO extends JpaRepository<Article,Integer> {
    //根据Id查找
    Article findById(int id);
    //查找一个作者的全部文章
    List<Article> findAllByAuthor(User author);
    //用于搜索的接口
    List<Article> findAllByTitleLike(String keyword);
}
