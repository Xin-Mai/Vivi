package com.example.vivi.service;

import com.example.vivi.dao.UserDAO;
import com.example.vivi.pojo.User;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

@Service
public class UserService {
    //自动从容器中找到一个对象
    @Autowired
    UserDAO userDAO;
    public boolean isExist(String username) {

        User user = getByName(username);
        return null!=user;
    }
    public User getById(int id) {return userDAO.findById(id);}

    public User getByName(String username) {
        return userDAO.findByUsername(username);
    }

    public User get(String username, String password){
        return userDAO.getByUsernameAndPassword(username, password);
    }

    public void add(User user) {
        userDAO.save(user);
    }
}
