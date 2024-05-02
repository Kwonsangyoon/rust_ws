struct Node
{
    x : f32,
    child_node: Option<Box<Node>>,
}

impl Node
{
    fn append(&mut self, elem: f32)
    {
        match self.child_node
        {
            Some(ref mut child_node) =>
            {child_node.append(elem);}
            None =>
            {
                let node = Node
                {
                    x: elem,
                    child_node : None,
                };
                self.child_node = Some(Box::new(node))
            }
        }
    }
    fn list(&self)
    {
        print!("{},",self.x);
        match self.child_node
        {
            Some(ref child_node) => child_node.list(),
            None => {}
        }
    }
}

fn main() {
    let mut head = Node
    {
        x : 1.0,
        child_node : None,
    };
    
    head.append(0.3);
    
    head.list();
}
