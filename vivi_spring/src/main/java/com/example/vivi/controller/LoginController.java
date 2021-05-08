package com.example.vivi.controller;

import com.example.vivi.pojo.User;
import com.example.vivi.result.Result;
import com.example.vivi.service.UserService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.util.HtmlUtils;

import javax.transaction.Transactional;
import java.util.Objects;

@Controller
public class LoginController {

    @Autowired
    UserService userService;

    @CrossOrigin
    @PostMapping(value = "/api/login")
    @ResponseBody
    @Transactional
    public Result login(@RequestBody User requestUser) {
        String username = requestUser.getUsername();
        username = HtmlUtils.htmlEscape(username);

        User user = userService.get(username, requestUser.getPassword());
        if (null == user) {
            System.out.println("wrong name or pw");
            return new Result(400);
        } else {
            userService.login(user);
            return new Result("id",user.getId());
            //return new Result(200);
        }
    }

    @CrossOrigin
    @GetMapping(value = "/api/logout/{id}")
    @ResponseBody
    @Transactional
    public Result logout(@PathVariable("id") int id) throws Exception{
        System.out.println("receive logout from"+id);
        User user = userService.getById(id);
        if (null == user){
            return new Result(400);
        }
        else{
            userService.logout(user);
            return new Result(200);
        }
    }
}