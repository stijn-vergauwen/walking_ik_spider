# walking_ik_spider
This was my first time working with IK in 3D, each leg keeps track of it's ideal leg location and when the leg moves too far away it get's replaced to a better position.

I learned a lot about quaternions and euler rotations, as well as about child-parent hierarchies, local and global transforms, and ofc inverse kinematics while building this project!

I wanted the spider to also turn but when writing the inverse kinematics code I didn't plan for this.  
So I decided to not implement turning in this project since it would likely require a lot of rewriting of code that already confused me haha.

To run this project you just need Rust installed.

![Screenshot from 2023-09-05 16-24-50](https://github.com/stijn-vergauwen/walking_ik_spider/assets/85249104/be63a373-0784-44a4-9e6d-de09acb6a816)
