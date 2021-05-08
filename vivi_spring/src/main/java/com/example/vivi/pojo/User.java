package com.example.vivi.pojo;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

import javax.persistence.*;
import java.util.List;

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
    //登录状态
    private Boolean state;
    //头像存储路径
    private String avatar;
    //自我介绍
    private String intro;

    //粉丝
    @OneToMany(fetch = FetchType.LAZY)
    @JoinTable(name = "follow", joinColumns = {@JoinColumn(name = "author")},
    inverseJoinColumns = {@JoinColumn(name = "follower")})
    private List<User> followers;
    //关注
    @OneToMany(fetch = FetchType.LAZY)
    @JoinTable(name = "follow", joinColumns = {@JoinColumn(name = "follower")},
            inverseJoinColumns = {@JoinColumn(name = "author")})
    private List<User> following;

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

    public Boolean getState() {
        return state;
    }

    public void setState(Boolean state) {
        this.state = state;
    }

    public String getAvatar() {
        return avatar;
    }

    public void setAvatar(String avatar) {
        this.avatar = avatar;
    }

    public List<User> getFollowers() {
        return followers;
    }

    public void setFollowers(List<User> followers) {
        this.followers = followers;
    }

    public List<User> getFollowing() {
        return following;
    }

    public void setFollowing(List<User> following) {
        this.following = following;
    }

    public String getIntro() {
        return intro;
    }

    public void setIntro(String intro) {
        this.intro = intro;
    }
}