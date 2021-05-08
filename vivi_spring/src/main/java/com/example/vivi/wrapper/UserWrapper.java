package com.example.vivi.wrapper;

import com.example.vivi.pojo.User;

public class UserWrapper {
    private int id;
    private String username;
    private String intro;
    private String avatar;

    public UserWrapper(User user){
        this.id = user.getId();
        this.username = user.getUsername();
        this.intro = user.getIntro();
        this.avatar = user.getAvatar();
    }

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

    public String getIntro() {
        return intro;
    }

    public void setIntro(String intro) {
        this.intro = intro;
    }

    public String getAvatar() {
        return avatar;
    }

    public void setAvatar(String avatar) {
        this.avatar = avatar;
    }
}
