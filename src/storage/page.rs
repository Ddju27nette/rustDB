pub type PageId = u64;
#[derive(Debug, Clone)]
/// Cette structure de données représente une page de données dans le système de stockage.
/// Elle contient un identifiant unique pour la page (id), les données elles-mêmes (data) et un indicateur pour savoir si la page a été modifiée (is_dirty).
/// L'identifiant de la page (id) est utilisé pour localiser la page sur le disque ou dans la mémoire, 
/// tandis que les données (data) contiennent le 
/// contenu réel de la page. L'indicateur is_dirty
///  est utilisé pour savoir si la page a été modifiée 
/// depuis sa dernière lecture ou écriture, ce qui est 
/// important pour la gestion de la mémoire et du stockage.
pub struct Page {
    pub id: PageId,
    pub data: [u8; 4096],
    pub is_dirty: bool,
}
impl  Page {
    pub fn new(id: PageId) -> Self {
        Self {
            id,
            data: [0; 4096],
            is_dirty: false,
        }
    }
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
}