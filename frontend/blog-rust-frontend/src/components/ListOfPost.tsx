import React, { useEffect, useState } from "react";

interface Post {
  id: string;
  name: string;
  description: string;
}
export const ListOfPost = ({ changes }: any) => {
  const [listOfPost, setListOfPost] = useState<Post[]>([]);
  useEffect(() => {
    const getPost = async () => {
      const response = await fetch("http://127.0.0.1:8080/");
      console.log(response);
      const result = await response.json();

      setListOfPost(result);
    };
    getPost();
  }, [changes]);
  console.log(listOfPost);
  return (
    <div>
      {listOfPost.map((post) => (
        <a href={`post/${post.id}`}>
          <h1 className="text-white">{post.name}</h1>
        </a>
      ))}
    </div>
  );
};
