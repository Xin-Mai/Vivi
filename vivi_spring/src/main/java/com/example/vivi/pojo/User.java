package com.example.vivi.pojo;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import javax.persistence.*;

//实体类
@Entity
//在数据库中对应user表
@Table(name = "user")
//在jpa进行持久化时忽略掉以下的属性
@JsonIgnoreProperties({"handler","hibernateLazyInitializer"})

public class User{
    //主键字段
    @Id
    //实体的主键由数据库生成
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    @Column(name="id")
    private int id;
    private String username;
    private String password;

    public int getId() {
        return id;
    }

    public void setId(int id) {
        this.id = id;
    }

    public String getUsername() {
        return username;
    }

    public void setUsername(String username) {
        this.username = username;
    }

    public String getPassword() {
        return password;
    }

    public void setPassword(String password) {
        this.password = password;
    }
}