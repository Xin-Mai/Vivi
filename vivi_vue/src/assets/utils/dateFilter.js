export default{
    filt(date){
        let pattern = /T(.*)/g;
        let new_date = date.replace(pattern,"");
        //console.log(new_date);
        return new_date;
    }
}