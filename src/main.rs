//! computing integrals with libcin0. this is
//! libcint/examples/c_call_cartesia0.c

#![allow(unused)]

use std::ptr::null_mut;

use libc::c_int;
use libcint_sys::{
    cint1e_ipnuc_cart, cint2e_cart, cint2e_cart_optimizer, CINTcgto_cart,
    CINTdel_optimizer, CINTgto_norm, ANG_OF, ATM_SLOTS, ATOM_OF, BAS_SLOTS,
    CHARGE_OF, NCTR_OF, NPRIM_OF, PTR_COEFF, PTR_COORD, PTR_ENV_START, PTR_EXP,
};

fn main() {
    let natm: c_int = 2;
    let nbas: c_int = 4;
    // atm_slots = 6; bas_slots = 8;
    let mut atm = vec![0; natm as usize * ATM_SLOTS as usize];
    let mut bas: Vec<c_int> = vec![0; nbas as usize * BAS_SLOTS as usize];
    let mut env = vec![0.0; 10000];

    let mut off = PTR_ENV_START as usize;

    let mut i = 0;
    atm[(CHARGE_OF + ATM_SLOTS * i) as usize] = 1;
    atm[(PTR_COORD + ATM_SLOTS * i) as usize] = off;
    // x, y, z in Bohr
    env[off + 0] = 0.0;
    env[off + 1] = 0.0;
    env[off + 2] = -0.8;

    i += 1;
    off += 3;

    atm[(CHARGE_OF + ATM_SLOTS * i) as usize] = 1;
    atm[(PTR_COORD + ATM_SLOTS * i) as usize] = off;
    // x, y, z in Bohr
    env[off + 0] = 0.0;
    env[off + 1] = 0.0;
    env[off + 2] = 0.8;
    i += 1;
    off += 3;

    let mut n = 0;

    /* basis #0, 3s -> 2s */
    bas[(ATOM_OF + BAS_SLOTS * n) as usize] = 0;
    bas[(ANG_OF + BAS_SLOTS * n) as usize] = 0;
    bas[(NPRIM_OF + BAS_SLOTS * n) as usize] = 3;
    bas[(NCTR_OF + BAS_SLOTS * n) as usize] = 2;
    bas[(PTR_EXP + BAS_SLOTS * n) as usize] = off as c_int;
    env[off + 0] = 6.0;
    env[off + 1] = 2.0;
    env[off + 2] = 0.8;
    off += 3;
    bas[(PTR_COEFF + BAS_SLOTS * n) as usize] = off as c_int;
    unsafe {
        env[off + 0] = 0.7
            * CINTgto_norm(
                bas[(ANG_OF + BAS_SLOTS * n) as usize] as c_int,
                env[(bas[(PTR_EXP + BAS_SLOTS * n) as usize] + 0) as usize],
            );
        env[off + 1] = 0.6
            * CINTgto_norm(
                bas[(ANG_OF + BAS_SLOTS * n) as usize] as c_int,
                env[(bas[(PTR_EXP + BAS_SLOTS * n) as usize] + 1) as usize],
            );
        env[off + 2] = 0.5
            * CINTgto_norm(
                bas[(ANG_OF + BAS_SLOTS * n) as usize] as c_int,
                env[(bas[(PTR_EXP + BAS_SLOTS * n) as usize] + 2) as usize],
            );
        env[off + 3] = 0.4
            * CINTgto_norm(
                bas[(ANG_OF + BAS_SLOTS * n) as usize] as c_int,
                env[(bas[(PTR_EXP + BAS_SLOTS * n) as usize] + 0) as usize],
            );
        env[off + 4] = 0.3
            * CINTgto_norm(
                bas[(ANG_OF + BAS_SLOTS * n) as usize] as c_int,
                env[(bas[(PTR_EXP + BAS_SLOTS * n) as usize] + 1) as usize],
            );
        env[off + 5] = 0.2
            * CINTgto_norm(
                bas[(ANG_OF + BAS_SLOTS * n) as usize] as c_int,
                env[(bas[(PTR_EXP + BAS_SLOTS * n) as usize] + 2) as usize],
            );
    }
    off += 6;
    n += 1;

    /* basis #1 */
    bas[(ATOM_OF + BAS_SLOTS * n) as usize] = 0;
    bas[(ANG_OF + BAS_SLOTS * n) as usize] = 1;
    bas[(NPRIM_OF + BAS_SLOTS * n) as usize] = 1;
    bas[(NCTR_OF + BAS_SLOTS * n) as usize] = 1;
    bas[(PTR_EXP + BAS_SLOTS * n) as usize] = off as c_int;
    env[off + 0] = 0.9;
    off += 1;
    bas[(PTR_COEFF + BAS_SLOTS * n) as usize] = off as c_int;
    unsafe {
        env[off + 0] = 1.
            * CINTgto_norm(
                bas[(ANG_OF + BAS_SLOTS * n) as usize] as c_int,
                env[(bas[(PTR_EXP + BAS_SLOTS * n) as usize]) as usize],
            );
    }
    off += 1;
    n += 1;

    /* basis #2 == basis #0 */
    bas[(ATOM_OF + BAS_SLOTS * n) as usize] = 1;
    bas[(ANG_OF + BAS_SLOTS * n) as usize] =
        bas[(ANG_OF + BAS_SLOTS * 0) as usize];
    bas[(NPRIM_OF + BAS_SLOTS * n) as usize] =
        bas[(NPRIM_OF + BAS_SLOTS * 0) as usize];
    bas[(NCTR_OF + BAS_SLOTS * n) as usize] =
        bas[(NCTR_OF + BAS_SLOTS * 0) as usize];
    bas[(PTR_EXP + BAS_SLOTS * n) as usize] =
        bas[(PTR_EXP + BAS_SLOTS * 0) as usize];
    bas[(PTR_COEFF + BAS_SLOTS * n) as usize] =
        bas[(PTR_COEFF + BAS_SLOTS * 0) as usize];
    n += 1;

    /* basis #3 == basis #1 */
    bas[(ATOM_OF + BAS_SLOTS * n) as usize] = 1;
    bas[(ANG_OF + BAS_SLOTS * n) as usize] =
        bas[(ANG_OF + BAS_SLOTS * 1) as usize];
    bas[(NPRIM_OF + BAS_SLOTS * n) as usize] =
        bas[(NPRIM_OF + BAS_SLOTS * 1) as usize];
    bas[(NCTR_OF + BAS_SLOTS * n) as usize] =
        bas[(NCTR_OF + BAS_SLOTS * 1) as usize];
    bas[(PTR_EXP + BAS_SLOTS * n) as usize] =
        bas[(PTR_EXP + BAS_SLOTS * 1) as usize];
    bas[(PTR_COEFF + BAS_SLOTS * n) as usize] =
        bas[(PTR_COEFF + BAS_SLOTS * 1) as usize];
    n += 1;

    /*
     * call one-electron cartesian integrals
     * the integral has 3 components, saving as
     * buf[      0:  di*dj]    for x
     * buf[  di*dj:2*di*dj]    for y
     * buf[2*di*dj:3*di*dj]    for z
     */

    let mut shls = vec![0; 4];
    unsafe {
        let mut i = 0;
        shls[0] = i;
        let mut di = CINTcgto_cart(i, bas.as_ptr().cast());
        let mut j = 1;
        shls[1] = j;
        let mut dj = CINTcgto_cart(j, bas.as_ptr().cast());
        let mut buf = vec![0.0; (di * dj) as usize * 3];
        if (0
            != cint1e_ipnuc_cart(
                buf.as_mut_ptr(),
                shls.as_mut_ptr().cast(),
                atm.as_mut_ptr().cast(),
                natm,
                bas.as_mut_ptr().cast(),
                nbas,
                env.as_mut_ptr().cast(),
            ))
        {
            print!("This gradient integral is not 0.\n");
        } else {
            print!("This integral is 0.\n");
        }

        /*
         * call two-electron cartesian integrals
         */
        i = 0;
        shls[0] = i;
        di = CINTcgto_cart(i, bas.as_ptr().cast());
        j = 1;
        shls[1] = j;
        dj = CINTcgto_cart(j, bas.as_ptr().cast());
        let mut k = 2;
        shls[2] = k;
        let mut dk = CINTcgto_cart(k, bas.as_ptr().cast());
        let mut l = 2;
        shls[3] = l;
        let mut dl = CINTcgto_cart(l, bas.as_ptr().cast());
        let mut buf = vec![0.0; (di * dj * dk * dl) as usize];
        if (0
            != cint2e_cart(
                buf.as_mut_ptr().cast(),
                shls.as_mut_ptr().cast(),
                atm.as_mut_ptr().cast(),
                natm,
                bas.as_mut_ptr().cast(),
                nbas,
                env.as_mut_ptr().cast(),
                null_mut(),
            ))
        {
            print!("This integral is not 0.\n");
        } else {
            print!("This integral is 0.\n");
        }

        let mut opt = null_mut();
        cint2e_cart_optimizer(
            &mut opt,
            atm.as_mut_ptr().cast(),
            natm,
            bas.as_mut_ptr().cast(),
            nbas,
            env.as_mut_ptr().cast(),
        );
        i = 0;
        shls[0] = i;
        di = CINTcgto_cart(i, bas.as_ptr().cast());
        j = 1;
        shls[1] = j;
        dj = CINTcgto_cart(j, bas.as_ptr().cast());
        k = 2;
        shls[2] = k;
        dk = CINTcgto_cart(k, bas.as_ptr().cast());
        l = 2;
        shls[3] = l;
        dl = CINTcgto_cart(l, bas.as_ptr().cast());
        let mut buf = vec![0.0; (di * dj * dk * dl) as usize];
        if (0
            != cint2e_cart(
                buf.as_mut_ptr().cast(),
                shls.as_mut_ptr().cast(),
                atm.as_mut_ptr().cast(),
                natm,
                bas.as_mut_ptr().cast(),
                nbas,
                env.as_mut_ptr().cast(),
                opt,
            ))
        {
            print!("This integral is not 0.\n");
        } else {
            print!("This integral is 0.\n");
        }
        CINTdel_optimizer(&mut opt);
    }
}
