# Tennis raquet theorem

Let the object have a moment of inertia $I_0=diag(I_1,I_2,I_3)$ in the coordinate system of its principal axes. Let us assume in the initial moment, the principal axes coincide with the world coordinate system. Once the object rotates by some rotation matrix $R$, meaning $x_{new} = R x_{old}$, the new moment of inertia is $I = R I_0 R^{-1}$. Therefore, the new angular velocity is 

$$\vec{\omega} = I^{-1}\vec{L} = (RI_0R^{-1})^{-1}\vec{L} = (R I_0^{-1} R^{-1})\vec{L}$$.

We can make small corrections to $\vec{\omega}$ here to make sure the initial energy is conserved TODO!

Knowing the angular velocity, the rotation matrix changes by $dR = \vec{\omega} \cross R dt$


