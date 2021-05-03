package com.example.vivi.service;

import com.example.vivi.dao.ArticleDAO;
import com.example.vivi.dao.CommentDAO;
import com.example.vivi.pojo.Article;
import com.example.vivi.pojo.Comment;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class CommentService {
    @Autowired
    CommentDAO commentDAO;
    @Autowired
    ArticleDAO articleDAO;

    //增加评论
    public void addComment(Comment comment){
        commentDAO.save(comment);
    }

    //删除评论
    public void deleteById(int id){
        commentDAO.deleteById(id);
    }

    //获取一篇文章的全部评论，按时间先后排序
    public List<Comment> getAllComments(int aid){
        Article article = articleDAO.findById(aid);
        return commentDAO.findAllByArticleOrderByPublishDateAsc(article);
    }
}
