export default{
    async codeGraph(file){
        let url;
        let coder = new Promise((resolve,reject)=>{
            let reader = new FileReader();
  		    reader.readAsDataURL(file);
  		    reader.onload = () => {
  		        resolve(reader.result);
            }
            setTimeout(()=>{
                resolve();
            },3000);
        }).then((result)=>{
            url=result;
        });
        await coder;
        return url;
    }
}
