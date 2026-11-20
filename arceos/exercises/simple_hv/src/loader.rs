use std::fs::File;
use std::io::{self, Read};

use alloc::vec::Vec;
//use axhal::mem::{phys_to_virt, PAGE_SIZE_4K};
use axhal::mem::PAGE_SIZE_4K;
use axhal::paging::MappingFlags;
use axmm::AddrSpace;

use crate::VM_ENTRY;

/*pub fn load_vm_image(fname: &str, uspace: &mut AddrSpace) -> io::Result<()> {
    let buf = load_file(fname)?;
    if buf.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "empty guest image",
        ));
    }

    let image_size = (buf.len() + PAGE_SIZE_4K - 1) & !(PAGE_SIZE_4K - 1);

    uspace
        .map_alloc(
            VM_ENTRY.into(),
            image_size,
            MappingFlags::READ
                | MappingFlags::WRITE
                | MappingFlags::EXECUTE
                | MappingFlags::USER,
            true,
        )
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "map guest image failed"))?;

    let (paddr, _, _) = uspace
        .page_table()
        .query(VM_ENTRY.into())
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "query guest image failed"))?;

    ax_println!("paddr: {:#x}", paddr);

    unsafe {
        core::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            phys_to_virt(paddr).as_mut_ptr(),
            buf.len(),
        );
    }

    Ok(())
}
*/
pub fn load_vm_image(fname: &str, uspace: &mut AddrSpace) -> io::Result<()> {
    let buf = load_file(fname)?;
    assert!(!buf.is_empty(), "empty guest image");

    let image_size = (buf.len() + PAGE_SIZE_4K - 1) & !(PAGE_SIZE_4K - 1);

    uspace
        .map_alloc(
            VM_ENTRY.into(),
            image_size,
            MappingFlags::READ
                | MappingFlags::WRITE
                | MappingFlags::EXECUTE
                | MappingFlags::USER,
            true,
        )
        .unwrap();

    let (paddr, _, _) = uspace
        .page_table()
        .query(VM_ENTRY.into())
        .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x}", VM_ENTRY));

    ax_println!("paddr: {:#x}", paddr);

    /*unsafe {
        core::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            phys_to_virt(paddr).as_mut_ptr(),
            buf.len(),
        );
    }*/
    uspace.write(VM_ENTRY.into(), &buf).unwrap();
    ax_println!("Guest image loaded");

    Ok(())
}

fn load_file(fname: &str) -> io::Result<Vec<u8>> {
    ax_println!("app: {}", fname);

    let mut file = File::open(fname)?;
    let mut buf = Vec::new();
    let mut chunk = [0u8; 512];

    loop {
        let n = file.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&chunk[..n]);
    }

    Ok(buf)
}

/*use std::io::{self, Read};
use std::fs::File;
use axhal::paging::MappingFlags;
use axhal::mem::{PAGE_SIZE_4K, phys_to_virt};
use axmm::AddrSpace;
use crate::VM_ENTRY;

pub fn load_vm_image(fname: &str, uspace: &mut AddrSpace) -> io::Result<()> {
    let mut buf = [0u8; 64];
    load_file(fname, &mut buf)?;

    uspace.map_alloc(VM_ENTRY.into(), PAGE_SIZE_4K, MappingFlags::READ|MappingFlags::WRITE|MappingFlags::EXECUTE|MappingFlags::USER, true).unwrap();

    let (paddr, _, _) = uspace
        .page_table()
        .query(VM_ENTRY.into())
        .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x}", VM_ENTRY));

    ax_println!("paddr: {:#x}", paddr);

    unsafe {
        core::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            phys_to_virt(paddr).as_mut_ptr(),
            PAGE_SIZE_4K,
        );
    }

    Ok(())
}

fn load_file(fname: &str, buf: &mut [u8]) -> io::Result<usize> {
    ax_println!("app: {}", fname);
    let mut file = File::open(fname)?;
    let n = file.read(buf)?;
    Ok(n)
}
*/