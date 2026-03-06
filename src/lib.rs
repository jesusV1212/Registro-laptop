use anchor_lang::prelude::*;

declare_id!("49DLAJWXSA4Z9nKwpkHEAKBpZwTWc2Uzi8g42iq9RbKo");

#[program]
pub mod laptop_registry {
    use super::*;

    
    pub fn inicializar_registro(ctx: Context<InicializarRegistro>) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        registro.owner = *ctx.accounts.owner.key;
        registro.inventario = Vec::new();
        Ok(())
    }

    
    pub fn agregar_laptop(
        ctx: Context<GestionarLaptop>,
        marca: String,
        modelo: String,
        procesador: String,
        serie: String,
        anio: u16,
    ) -> Result<()> {
        let registro = &mut ctx.accounts.registro;

       
        if registro.inventario.iter().any(|l| l.serie == serie) {
            return Err(ErrorCode::LaptopDuplicada.into());
        }

       
        if anio < 1970 || anio > 2026 {
            return Err(ErrorCode::AnioInvalido.into());
        }

        let nueva_laptop = Laptop { marca, modelo, procesador, serie, anio };
        registro.inventario.push(nueva_laptop);
        msg!("Laptop registrada exitosamente.");
        Ok(())
    }

   
    pub fn ver_registros(ctx: Context<VerRegistros>) -> Result<()> {
        let registro = &ctx.accounts.registro;

        if registro.inventario.is_empty() {
            msg!("El inventario de laptops está vacío.");
        } else {
            msg!("--- INVENTARIO DE LAPTOPS REGISTRADAS ---");
            for (i, laptop) in registro.inventario.iter().enumerate() {
                msg!(
                    "Laptop #{}: {} {} | CPU: {} | Serie: {} | Año: {}",
                    i + 1,
                    laptop.marca,
                    laptop.modelo,
                    laptop.procesador,
                    laptop.serie,
                    laptop.anio
                );
            }
        }
        Ok(())
    }

    
    pub fn actualizar_laptop(
        ctx: Context<GestionarLaptop>, 
        modelo: String, 
        nuevo_procesador: String
    ) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        
        if let Some(laptop) = registro.inventario.iter_mut().find(|l| l.modelo == modelo) {
            laptop.procesador = nuevo_procesador;
            msg!("Actualización exitosa: El modelo {} ahora tiene CPU {}", modelo, laptop.procesador);
            Ok(())
        } else {
            Err(ErrorCode::LaptopNoEncontrada.into())
        }
    }

   
    pub fn eliminar_laptop(ctx: Context<GestionarLaptop>, modelo: String) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        if let Some(pos) = registro.inventario.iter().position(|l| l.modelo == modelo) {
            registro.inventario.swap_remove(pos);
            msg!("Laptop {} eliminada del sistema.", modelo);
            Ok(())
        } else {
            Err(ErrorCode::LaptopNoEncontrada.into())
        }
    }
}



#[account]
#[derive(InitSpace)]
pub struct RegistroLaptops {
    pub owner: Pubkey,
    #[max_len(10)] 
    pub inventario: Vec<Laptop>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Laptop {
    #[max_len(30)] pub marca: String,
    #[max_len(30)] pub modelo: String,
    #[max_len(30)] pub procesador: String,
    #[max_len(20)] pub serie: String,
    pub anio: u16,
}

#[derive(Accounts)]
pub struct InicializarRegistro<'info> {
    #[account(init, payer = owner, space = 8 + RegistroLaptops::INIT_SPACE, seeds = [b"registro", owner.key().as_ref()], bump)]
    pub registro: Account<'info, RegistroLaptops>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarLaptop<'info> {
    #[account(mut, has_one = owner)]
    pub registro: Account<'info, RegistroLaptops>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerRegistros<'info> {
    pub registro: Account<'info, RegistroLaptops>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("La laptop especificada no existe en el inventario.")]
    LaptopNoEncontrada,
    #[msg("Ya existe una laptop con este número de serie.")]
    LaptopDuplicada,
    #[msg("El año debe estar entre 1970 y 2026.")]
    AnioInvalido,
}
