package com.example.vivi.controller;

import com.example.vivi.pojo.Article;
import com.example.vivi.pojo.User;
import com.example.vivi.service.ArticleService;
import com.example.vivi.service.UserService;
import com.example.vivi.wrapper.ArticleWrapper;
import com.example.vivi.wrapper.UserWrapper;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.*;

import java.util.ArrayList;
import java.util.List;

@Controller
public class UserCenterController {
    @Autowired
    UserService userService;
    @Autowired
    ArticleService articleService;

    @CrossOrigin
    @ResponseBody
    @GetMapping(value = "/api/usercenter/{id}")
    public int[] userCenterHeader(@PathVariable("id") int id) throws Exception{
        int[] data = new int[5];
        User user = userService.getById(id);
        if (user!=null){
            //关注
            data[0] =  user.getFollowing().size();
            //粉丝
            data[1] = user.getFollowers().size();
            //文章
            data[2] = articleService.getAllArticlesByAuthor(id).size();
            //阅读量
            data[3] = articleService.getTotalReadByAuthor(id);
            //喜欢
            data[4] = articleService.getTotalLikeByAuthor(id);
        }
        return data;
    }

    @CrossOrigin
    @ResponseBody
    @GetMapping(value = "/api/{id}/following")
    public List<UserWrapper> getFollowing(@PathVariable("id") int id) throws Exception{
        User user = userService.getById(id);
        List<UserWrapper> followings = new ArrayList<>();
        if (user!=null){
            List<User> list = user.getFollowing();
            for(int i=0;i<list.size();i++){ followings.add(new UserWrapper(list.get(i))); }
        }
        return followings;
    }

    @CrossOrigin
    @ResponseBody
    @GetMapping(value = "/api/{id}/follower")
    public List<UserWrapper> getFollower(@PathVariable("id") int id) throws Exception{
        User user = userService.getById(id);
        List<UserWrapper> followers = new ArrayList<>();
        if (user!=null){
            List<User> list = user.getFollowers();
            for(int i=0;i<list.size();i++){ followers.add(new UserWrapper(list.get(i))); }
        }
        return followers;
    }

    @CrossOrigin
    @ResponseBody
    @GetMapping(value = "/api/{id}/articles")
    public List<ArticleWrapper> getArticlesByAuthor(@PathVariable("id") int id) throws Exception{
        List<ArticleWrapper> articles = new ArrayList<>();
        List<Article> list = articleService.getAllArticlesByAuthor(id);
        if (list!=null){
            for(int i=0;i<list.size();i++) {articles.add(new ArticleWrapper(list.get(i)));}
        }
        return articles;
    }

}
