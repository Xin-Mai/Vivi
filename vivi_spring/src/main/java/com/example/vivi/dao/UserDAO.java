package com.example.vivi.dao;

import com.example.vivi.pojo.User;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Modifying;
import org.springframework.data.jpa.repository.Query;

public interface UserDAO extends JpaRepository<User,Integer> {
    User findById(int id);
    User findByUsername(String username);
    User getByUsernameAndPassword(String username,String password);
    //修改登录状态
    @Modifying
    @Query("update User u set u.state = ?1 where u.id = ?2")
    int updateStateById(Boolean state, int id);
}
