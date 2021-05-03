package com.example.vivi.dao;


import com.example.vivi.pojo.Article;
import com.example.vivi.pojo.Comment;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;

public interface CommentDAO extends JpaRepository<Comment, Integer> {
    //获取一篇文章的全部评论，根据先后排序
    List<Comment> findAllByArticleOrderByPublishDateAsc(Article article);
}
